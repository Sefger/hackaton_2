use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- Базовые типы сообщений и чатов ---
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub thread_sn: Option<String>,
    pub time: i64,
    pub text: String,
    pub sender_id: String,
    pub file_snippets: Option<String>,
    pub parts: Option<Vec<HashMap<String, serde_json::Value>>>,
    pub mentions: Option<Vec<String>>,
    pub member_event: Option<HashMap<String, serde_json::Value>>,
    pub is_system: bool,
    pub is_hidden: bool,
    pub is_forward: bool,
    pub is_quote: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Private,
    Group,
    Channel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
    pub id: String,
    pub name: String,
    pub sn: String,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    pub is_public: Option<bool>,
    pub members_count: Option<i32>,
    pub members: Option<Vec<HashMap<String, serde_json::Value>>>,
}

// --- Index Service API ---
#[derive(Debug, Deserialize, Serialize)]
pub struct IndexAPIDataItem {
    pub chat: Chat,
    pub overlap_messages: Vec<Message>,
    pub new_messages: Vec<Message>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexAPIRequest {
    pub data: IndexAPIDataItem,
}

#[derive(Debug, Serialize, Deserialize)]
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

// --- Search Service API ---
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub search_text: String,
    // ... остальные поля из твоего кода
}

#[derive(Debug, Deserialize, Serialize)]
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
