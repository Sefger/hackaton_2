use async_trait::async_trait;
use qdrant_client::qdrant::{
    Fusion, PrefetchQueryBuilder, Query, QueryPointsBuilder, Value, VectorInput,
};
use qdrant_client::Qdrant;

use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct Candidate {
    pub id: String,
    pub page_content: String,
    pub message_ids: Vec<String>,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct SparseQuery {
    pub indices: Vec<u32>,
    pub values: Vec<f32>,
}

#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn hybrid_search(
        &self,
        dense: Vec<f32>,
        sparse: SparseQuery,
        limit: u64,
    ) -> Result<Vec<Candidate>, AppError>;
}

pub struct QdrantStore {
    pub client: Qdrant,
    pub collection: String,
    pub dense_vec_name: String,
    pub sparse_vec_name: String,
}

#[async_trait]
impl VectorStore for QdrantStore {
    async fn hybrid_search(
        &self,
        dense: Vec<f32>,
        sparse: SparseQuery,
        limit: u64,
    ) -> Result<Vec<Candidate>, AppError> {
        // Build sparse vector input from parallel vecs by zipping into a slice of (u32, f32).
        let sparse_pairs: Vec<(u32, f32)> = sparse
            .indices
            .into_iter()
            .zip(sparse.values.into_iter())
            .collect();
        let sparse_input: VectorInput = VectorInput::from(sparse_pairs.as_slice());

        let dense_prefetch = PrefetchQueryBuilder::default()
            .query(Query::new_nearest(dense))
            .using(self.dense_vec_name.clone())
            .limit(limit)
            .build();

        let sparse_prefetch = PrefetchQueryBuilder::default()
            .query(Query::new_nearest(sparse_input))
            .using(self.sparse_vec_name.clone())
            .limit(limit)
            .build();

        let req = QueryPointsBuilder::new(&self.collection)
            .add_prefetch(dense_prefetch)
            .add_prefetch(sparse_prefetch)
            .query(Query::new_fusion(Fusion::Rrf))
            .limit(limit)
            .with_payload(true)
            .build();

        let resp = self
            .client
            .query(req)
            .await
            .map_err(|e| AppError::Qdrant(e.to_string()))?;

        let candidates = resp
            .result
            .into_iter()
            .map(|p| {
                let id = p
                    .id
                    .as_ref()
                    .map(|x| format!("{:?}", x))
                    .unwrap_or_default();
                let page_content = extract_string(&p.payload, "page_content");
                let message_ids = extract_string_list(&p.payload, "message_ids");
                Candidate { id, page_content, message_ids, score: p.score }
            })
            .collect();
        Ok(candidates)
    }
}

fn extract_string(payload: &std::collections::HashMap<String, Value>, key: &str) -> String {
    use qdrant_client::qdrant::value::Kind;
    payload
        .get(key)
        .and_then(|v| v.kind.as_ref())
        .and_then(|k| match k {
            Kind::StringValue(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_default()
}

fn extract_string_list(
    payload: &std::collections::HashMap<String, Value>,
    key: &str,
) -> Vec<String> {
    use qdrant_client::qdrant::value::Kind;
    let mut out = Vec::new();
    if let Some(v) = payload.get(key) {
        match &v.kind {
            Some(Kind::ListValue(lst)) => {
                for item in &lst.values {
                    if let Some(Kind::StringValue(s)) = &item.kind {
                        out.push(s.clone());
                    }
                }
            }
            Some(Kind::StringValue(s)) => out.push(s.clone()),
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod extract_tests {
    use super::*;
    use qdrant_client::qdrant::{value::Kind, ListValue, Value as QValue};
    use std::collections::HashMap;

    fn qval_string(s: &str) -> QValue {
        QValue { kind: Some(Kind::StringValue(s.to_string())) }
    }

    fn qval_list(items: Vec<&str>) -> QValue {
        QValue {
            kind: Some(Kind::ListValue(ListValue {
                values: items.into_iter().map(qval_string).collect(),
            })),
        }
    }

    #[test]
    fn extracts_string() {
        let mut p = HashMap::new();
        p.insert("page_content".into(), qval_string("hi"));
        assert_eq!(extract_string(&p, "page_content"), "hi");
    }

    #[test]
    fn extracts_missing_string_as_empty() {
        let p = HashMap::new();
        assert_eq!(extract_string(&p, "nope"), "");
    }

    #[test]
    fn extracts_list() {
        let mut p = HashMap::new();
        p.insert("message_ids".into(), qval_list(vec!["a", "b"]));
        assert_eq!(extract_string_list(&p, "message_ids"), vec!["a", "b"]);
    }

    #[test]
    fn extracts_single_string_as_list_of_one() {
        let mut p = HashMap::new();
        p.insert("message_ids".into(), qval_string("solo"));
        assert_eq!(extract_string_list(&p, "message_ids"), vec!["solo"]);
    }
}
