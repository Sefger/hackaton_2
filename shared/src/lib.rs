use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    #[serde(default)]
    pub thread_sn: Option<String>,
    pub time: i64,
    pub text: String,
    pub sender_id: String,
    #[serde(default)]
    pub file_snippets: String,
    #[serde(default)]
    pub parts: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub mentions: Vec<String>,
    #[serde(default)]
    pub member_event: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub is_system: bool,
    #[serde(default)]
    pub is_hidden: bool,
    #[serde(default)]
    pub is_forward: bool,
    #[serde(default)]
    pub is_quote: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Private,
    Group,
    Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: String,
    pub name: String,
    pub sn: String,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    #[serde(default)]
    pub is_public: Option<bool>,
    #[serde(default)]
    pub members_count: Option<i32>,
    #[serde(default)]
    pub members: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexAPIDataItem {
    pub chat: Chat,
    #[serde(default)]
    pub overlap_messages: Vec<Message>,
    #[serde(default)]
    pub new_messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexAPIRequest {
    pub data: IndexAPIDataItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexAPIResultItem {
    pub page_content: String,
    pub dense_content: String,
    pub sparse_content: String,
    pub message_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexAPIResponse {
    pub results: Vec<IndexAPIResultItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SparseVector {
    pub indices: Vec<u32>,
    pub values: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SparseEmbeddingRequest {
    pub texts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SparseEmbeddingResponse {
    pub vectors: Vec<SparseVector>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Entities {
    #[serde(default)]
    pub people: Vec<String>,
    #[serde(default)]
    pub emails: Vec<String>,
    #[serde(default)]
    pub documents: Vec<String>,
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(default)]
    pub links: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub text: String,
    #[serde(default)]
    pub asker: String,
    #[serde(default)]
    pub asked_on: String,
    #[serde(default)]
    pub variants: Vec<String>,
    #[serde(default)]
    pub hyde: Vec<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub entities: Entities,
    #[serde(default)]
    pub date_mentions: Vec<String>,
    #[serde(default)]
    pub date_range: Option<DateRange>,
    pub search_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAPIRequest {
    pub question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAPIResultItem {
    pub message_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAPIResponse {
    pub results: Vec<SearchAPIResultItem>,
}
