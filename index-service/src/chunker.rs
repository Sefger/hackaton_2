use shared::{Message, IndexAPIResultItem, IndexAPIDataItem};

pub struct ChunkerConfig {
    pub window_size: usize, // Сколько сообщений в одном чанке
    pub overlap: usize,    // Сколько сообщений перекрывается с предыдущим
}

/// Точка входа для handlers.rs
pub fn process_to_chunks(data: IndexAPIDataItem) -> Vec<IndexAPIResultItem> {
    // Настраиваем параметры окна.
    // Можно вынести в конфиг сервиса, но для начала хватит констант.
    let config = ChunkerConfig {
        window_size: 20, // Берем по 20 сообщений
        overlap: 5,      // 5 сообщений перекрытия для сохранения контекста
    };

    create_chunks(&data.new_messages, &data.overlap_messages, &config)
}

pub fn create_chunks(
    new_messages: &[Message],
    overlap_messages: &[Message],
    config: &ChunkerConfig,
) -> Vec<IndexAPIResultItem> {
    // Объединяем старые и новые сообщения для сохранения контекста
    let mut all_messages = Vec::new();
    all_messages.extend_from_slice(overlap_messages);
    all_messages.extend_from_slice(new_messages);

    if all_messages.is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();
    let mut i = 0;

    // Оптимизация: убрали лишний .to_vec() в цикле
    while i < all_messages.len() {
        let end = (i + config.window_size).min(all_messages.len());
        let chunk_msgs = &all_messages[i..end];

        if chunk_msgs.is_empty() { break; }

        let mut page_content = String::new();
        let mut message_ids = Vec::new();

        for msg in chunk_msgs {
            message_ids.push(msg.id.clone());
            let line = format!("{}: {}\n", msg.sender_id, msg.text.trim());
            page_content.push_str(&line);
        }

        results.push(IndexAPIResultItem {
            dense_content: page_content.clone(),
            sparse_content: page_content.clone(),
            page_content,
            message_ids,
        });

        if end == all_messages.len() { break; }

        // Сдвиг окна
        let step = if config.window_size > config.overlap {
            config.window_size - config.overlap
        } else {
            1
        };
        i += step;
    }

    // Фильтруем: оставляем только те чанки, где есть хотя бы одно НОВОЕ сообщение
    let new_ids: std::collections::HashSet<_> = new_messages.iter().map(|m| &m.id).collect();
    results.into_iter()
        .filter(|chunk| chunk.message_ids.iter().any(|id| new_ids.contains(id)))
        .collect()
}