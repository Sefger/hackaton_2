use std::sync::Arc;
use crate::AppState;
use shared::{IndexAPIDataItem, Message, SparseEmbeddingRequest, SparseEmbeddingResponse};
use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder, PointId};
use qdrant_client::Payload;
use anyhow::{Context, Result};
use uuid::Uuid;

/// Основной конвейер индексации сообщений
pub struct IndexingPipeline {
    state: Arc<AppState>,
}

impl IndexingPipeline {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    /// Запускает процесс преобразования сообщений в векторы и их сохранение
    pub async fn run(&self, data: IndexAPIDataItem) -> Result<()> {
        let chat_id = data.chat.id.clone();

        // 1. Подготовка текстов для эмбеддинга
        // Используем форматирование, специфичное для модели BGE-M3 (контекст отправителя)
        let texts: Vec<String> = data.new_messages
            .iter()
            .map(|m| self.format_message_for_index(m))
            .collect();

        if texts.is_empty() {
            return Ok(());
        }

        // 2. Запрос разреженных (sparse) векторов у Index Service
        let embedding_data = self.fetch_embeddings(texts).await?;

        // 3. Формирование точек (Points) для векторной БД
        let points = self.build_points(&chat_id, &data.new_messages, embedding_data)?;

        // 4. Пакетная вставка в Qdrant (коллекция "messages")
        self.state.qdrant
            .upsert_points(UpsertPointsBuilder::new("messages", points).wait(true))
            .await
            .context("Failed to upsert points to Qdrant")?;

        Ok(())
    }

    /// Взаимодействие с внешним сервисом эмбеддингов
    async fn fetch_embeddings(&self, texts: Vec<String>) -> Result<SparseEmbeddingResponse> {
        let index_url = format!("{}/sparse_embedding", self.state.index_service_url);

        let resp = self.state.http_client
            .post(&index_url)
            .json(&SparseEmbeddingRequest { texts })
            .send()
            .await
            .context("Failed to connect to index-service")?;

        if !resp.status().is_success() {
            let error_msg = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Index-service returned error: {}", error_msg));
        }

        resp.json::<SparseEmbeddingResponse>()
            .await
            .context("Failed to parse sparse embeddings response")
    }

    /// Преобразование сообщений и векторов в структуру Qdrant
    fn build_points(
        &self,
        chat_id: &str,
        messages: &[Message],
        embeddings: SparseEmbeddingResponse
    ) -> Result<Vec<PointStruct>> {
        let mut points = Vec::with_capacity(messages.len());

        for (idx, msg) in messages.iter().enumerate() {
            let sparse_vec = embeddings.vectors.get(idx)
                .context("Embedding vector index out of bounds")?;

            let mut payload = Payload::new();
            payload.insert("chat_id", chat_id.to_string());
            payload.insert("message_id", msg.id.clone());
            payload.insert("text", msg.text.clone());
            payload.insert("sender_id", msg.sender_id.clone());
            // Храним ID как список для совместимости с будущей логикой поиска по чанкам
            payload.insert("message_ids", vec![msg.id.clone()]);

            // Генерация PointId: пытаемся использовать UUID сообщения из базы,
            // если формат не совпадает — генерируем случайный v4
            let point_id: PointId = Uuid::parse_str(&msg.id)
                .map(|u| PointId::from(u.to_string()))
                .unwrap_or_else(|_| PointId::from(Uuid::new_v4().to_string()));

            points.push(PointStruct::new(
                point_id,
                sparse_vec.values.clone(),
                payload
            ));
        }

        Ok(points)
    }

    /// Форматирование сообщения для улучшения семантического поиска
    fn format_message_for_index(&self, msg: &Message) -> String {
        // Добавление "User {id}" помогает модели различать участников диалога
        format!("User {}: {}", msg.sender_id, msg.text.trim())
    }
}