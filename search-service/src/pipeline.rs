use std::sync::Arc;
use crate::AppState;
use shared::{IndexAPIDataItem, Message, SparseEmbeddingRequest, SparseEmbeddingResponse};
use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder};
use qdrant_client::Payload;

pub struct IndexingPipeline {
    state: Arc<AppState>,
}

impl IndexingPipeline {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn run(&self, data: IndexAPIDataItem) -> anyhow::Result<()> {
        let chat_id = data.chat.id.clone();

        // 1. Подготовка текстов для эмбеддинга
        // Мы склеиваем сообщения в чанки или обрабатываем по одному
        let texts: Vec<String> = data.new_messages
            .iter()
            .map(|m| self.format_message_for_index(m))
            .collect();

        if texts.is_empty() { return Ok(()); }

        // 2. Получение разреженных векторов от index-service
        let index_url = format!("{}/sparse_embedding", self.state.index_service_url);
        let resp = self.state.http_client
            .post(&index_url)
            .json(&SparseEmbeddingRequest { texts })
            .send()
            .await?;

        let embedding_data: SparseEmbeddingResponse = resp.json().await?;

        // 3. Формирование точек для Qdrant
        let mut points = Vec::new();
        for (idx, msg) in data.new_messages.iter().enumerate() {
            let vector = &embedding_data.vectors[idx];

            // Метаданные для сохранения
            let mut payload = Payload::new();
            payload.insert("chat_id", chat_id.clone());
            payload.insert("message_id", msg.id.clone());
            payload.insert("sender_id", msg.sender_id.clone());
            payload.insert("text", msg.text.clone());

            // Создаем точку (ID точки должен быть уникальным, можно использовать UUID из msg.id)
            let point = PointStruct::new(
                msg.id.clone(), // Если ID — это String, возможно нужно будет хешировать в u64 или UUID
                vector.values.iter().map(|&v| v as f32).collect::<Vec<_>>(),
                payload
            );
            points.push(point);
        }

        // 4. Запись в Qdrant
        self.state.qdrant
            .upsert_points(UpsertPointsBuilder::new("messages", points))
            .await?;

        Ok(())
    }

    fn format_message_for_index(&self, msg: &Message) -> String {
        // Здесь мы можем учитывать лимит в 8192 токена.
        // Для BGE-M3 полезно добавлять контекст, например, кто написал.
        format!("Sender: {}. Message: {}", msg.sender_id, msg.text)
    }
}