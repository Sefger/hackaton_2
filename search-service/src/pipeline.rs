use std::collections::HashSet;
use std::sync::Arc;

use shared::{Question, SearchAPIResponse, SearchAPIResultItem};

use crate::errors::AppError;
use crate::retrieval::qdrant::{Candidate, SparseQuery, VectorStore};

pub const CANDIDATE_LIMIT: u64 = 100;
pub const FINAL_LIMIT: usize = 50;

pub struct PipelineDeps<'a> {
    pub store: &'a Arc<dyn VectorStore>,
    pub dense_embed: Box<dyn Fn(&str) -> futures::future::BoxFuture<'a, Result<Vec<f32>, AppError>> + Send + Sync + 'a>,
    pub sparse_embed: Box<dyn Fn(&str) -> futures::future::BoxFuture<'a, Result<SparseQuery, AppError>> + Send + Sync + 'a>,
    pub rerank: Box<dyn Fn(&str, &[String]) -> futures::future::BoxFuture<'a, Vec<usize>> + Send + Sync + 'a>,
}

pub async fn execute(deps: &PipelineDeps<'_>, question: &Question) -> Result<SearchAPIResponse, AppError> {
    let query = if !question.search_text.trim().is_empty() {
        question.search_text.as_str()
    } else {
        question.text.as_str()
    };

    let dense_fut = (deps.dense_embed)(query);
    let sparse_fut = (deps.sparse_embed)(query);
    let (dense, sparse) = futures::future::try_join(dense_fut, sparse_fut).await?;

    let candidates = deps.store.hybrid_search(dense, sparse, CANDIDATE_LIMIT).await?;
    let docs: Vec<String> = candidates.iter().map(|c| c.page_content.clone()).collect();
    let order = (deps.rerank)(query, &docs).await;

    let message_ids = dedup_take(&candidates, &order, FINAL_LIMIT);
    Ok(SearchAPIResponse {
        results: vec![SearchAPIResultItem { message_ids }],
    })
}

fn dedup_take(candidates: &[Candidate], order: &[usize], limit: usize) -> Vec<String> {
    let mut out = Vec::with_capacity(limit);
    let mut seen = HashSet::new();
    for &idx in order {
        if let Some(c) = candidates.get(idx) {
            for id in &c.message_ids {
                if out.len() >= limit { return out; }
                if seen.insert(id.clone()) {
                    out.push(id.clone());
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct StubStore {
        results: Vec<Candidate>,
    }

    #[async_trait]
    impl VectorStore for StubStore {
        async fn hybrid_search(
            &self,
            _dense: Vec<f32>,
            _sparse: SparseQuery,
            _limit: u64,
        ) -> Result<Vec<Candidate>, AppError> {
            Ok(self.results.clone())
        }
    }

    fn cand(id: &str, msgs: &[&str]) -> Candidate {
        Candidate {
            id: id.into(),
            page_content: format!("content for {id}"),
            message_ids: msgs.iter().map(|s| s.to_string()).collect(),
            score: 0.5,
        }
    }

    fn q(text: &str) -> Question {
        Question {
            text: text.into(),
            asker: String::new(),
            asked_on: String::new(),
            variants: vec![],
            hyde: vec![],
            keywords: vec![],
            entities: Default::default(),
            date_mentions: vec![],
            date_range: None,
            search_text: text.into(),
        }
    }

    #[tokio::test]
    async fn happy_path_returns_reranked_ids() {
        let store: Arc<dyn VectorStore> = Arc::new(StubStore {
            results: vec![
                cand("p1", &["m1", "m2"]),
                cand("p2", &["m3"]),
                cand("p3", &["m4"]),
            ],
        });
        let deps = PipelineDeps {
            store: &store,
            dense_embed: Box::new(|_q| Box::pin(async { Ok(vec![0.0; 4]) })),
            sparse_embed: Box::new(|_q| Box::pin(async {
                Ok(SparseQuery { indices: vec![1], values: vec![1.0] })
            })),
            // rerank reverses the order
            rerank: Box::new(|_q, docs| {
                let n = docs.len();
                Box::pin(async move { (0..n).rev().collect() })
            }),
        };
        let resp = execute(&deps, &q("hi")).await.unwrap();
        assert_eq!(resp.results.len(), 1);
        // Reversed: p3, p2, p1 → m4, m3, m1, m2
        assert_eq!(resp.results[0].message_ids, vec!["m4", "m3", "m1", "m2"]);
    }

    #[tokio::test]
    async fn dedup_message_ids_across_candidates() {
        let store: Arc<dyn VectorStore> = Arc::new(StubStore {
            results: vec![
                cand("p1", &["m1", "m2"]),
                cand("p2", &["m2", "m3"]),
            ],
        });
        let deps = PipelineDeps {
            store: &store,
            dense_embed: Box::new(|_| Box::pin(async { Ok(vec![0.0]) })),
            sparse_embed: Box::new(|_| Box::pin(async {
                Ok(SparseQuery { indices: vec![], values: vec![] })
            })),
            rerank: Box::new(|_, docs| {
                let n = docs.len();
                Box::pin(async move { (0..n).collect() })
            }),
        };
        let resp = execute(&deps, &q("x")).await.unwrap();
        assert_eq!(resp.results[0].message_ids, vec!["m1", "m2", "m3"]);
    }

    #[tokio::test]
    async fn truncates_to_50() {
        let mut results = Vec::new();
        for i in 0..60 {
            results.push(cand(&format!("p{i}"), &[&format!("m{i}")]));
        }
        let store: Arc<dyn VectorStore> = Arc::new(StubStore { results });
        let deps = PipelineDeps {
            store: &store,
            dense_embed: Box::new(|_| Box::pin(async { Ok(vec![0.0]) })),
            sparse_embed: Box::new(|_| Box::pin(async {
                Ok(SparseQuery { indices: vec![], values: vec![] })
            })),
            rerank: Box::new(|_, docs| {
                let n = docs.len();
                Box::pin(async move { (0..n).collect() })
            }),
        };
        let resp = execute(&deps, &q("x")).await.unwrap();
        assert_eq!(resp.results[0].message_ids.len(), 50);
    }

    #[tokio::test]
    async fn empty_store_returns_empty_results() {
        let store: Arc<dyn VectorStore> = Arc::new(StubStore { results: vec![] });
        let deps = PipelineDeps {
            store: &store,
            dense_embed: Box::new(|_| Box::pin(async { Ok(vec![0.0]) })),
            sparse_embed: Box::new(|_| Box::pin(async {
                Ok(SparseQuery { indices: vec![], values: vec![] })
            })),
            rerank: Box::new(|_, docs| {
                let n = docs.len();
                Box::pin(async move { (0..n).collect() })
            }),
        };
        let resp = execute(&deps, &q("x")).await.unwrap();
        assert!(resp.results[0].message_ids.is_empty());
    }

    #[tokio::test]
    async fn falls_back_to_text_when_search_text_empty() {
        let store: Arc<dyn VectorStore> = Arc::new(StubStore {
            results: vec![cand("p1", &["m1"])],
        });
        let captured = std::sync::Mutex::new(String::new());
        let captured_ref = &captured;
        let deps = PipelineDeps {
            store: &store,
            dense_embed: Box::new(move |q| {
                *captured_ref.lock().unwrap() = q.to_string();
                Box::pin(async { Ok(vec![0.0]) })
            }),
            sparse_embed: Box::new(|_| Box::pin(async {
                Ok(SparseQuery { indices: vec![], values: vec![] })
            })),
            rerank: Box::new(|_, docs| {
                let n = docs.len();
                Box::pin(async move { (0..n).collect() })
            }),
        };
        let mut question = q("");
        question.text = "fallback-query".into();
        question.search_text = "".into();
        let _ = execute(&deps, &question).await.unwrap();
        assert_eq!(*captured.lock().unwrap(), "fallback-query");
    }
}
