use shared::*;

#[test]
fn test_question_deserializes_full_tz_sample() {
    let json = r#"{
        "text": "Кто отвечает за деплой?",
        "asker": "alice@example.com",
        "asked_on": "2025-10-20",
        "variants": ["ответственный за деплой", "deploy owner"],
        "hyde": ["Деплой курирует команда DevOps"],
        "keywords": ["деплой", "owner"],
        "entities": {
            "people": ["Bob"],
            "emails": ["bob@example.com"],
            "documents": ["RFC-42"],
            "names": ["CI"],
            "links": ["https://wiki"]
        },
        "date_mentions": ["вчера"],
        "date_range": { "from": "2025-10-19T00:00:00Z", "to": "2025-10-20T23:59:59Z" },
        "search_text": "кто отвечает за деплой owner devops"
    }"#;
    let q: Question = serde_json::from_str(json).expect("must parse");
    assert_eq!(q.text, "Кто отвечает за деплой?");
    assert_eq!(q.variants.len(), 2);
    assert_eq!(q.entities.people, vec!["Bob"]);
    assert_eq!(q.date_range.as_ref().unwrap().from, "2025-10-19T00:00:00Z");
    assert_eq!(q.search_text, "кто отвечает за деплой owner devops");
}

#[test]
fn test_question_deserializes_minimal() {
    let json = r#"{ "text": "hi", "search_text": "hi" }"#;
    let q: Question = serde_json::from_str(json).expect("must parse minimal");
    assert_eq!(q.text, "hi");
    assert!(q.variants.is_empty());
    assert!(q.hyde.is_empty());
    assert!(q.date_range.is_none());
    assert!(q.entities.people.is_empty());
}

#[test]
fn test_chat_type_snake_case() {
    let json = r#"{"id":"c","name":"n","sn":"s","type":"group"}"#;
    let c: Chat = serde_json::from_str(json).expect("must parse");
    assert!(matches!(c.chat_type, ChatType::Group));
}

#[test]
fn test_message_defaults_on_missing_optionals() {
    let json = r#"{"id":"m1","time":1,"text":"hello","sender_id":"u1","is_system":false,"is_hidden":false,"is_forward":false,"is_quote":false}"#;
    let m: Message = serde_json::from_str(json).expect("must parse");
    assert_eq!(m.id, "m1");
    assert_eq!(m.file_snippets, "");
    assert!(m.mentions.is_empty());
    assert!(m.parts.is_empty());
    assert!(m.member_event.is_none());
    assert!(m.thread_sn.is_none());
}

#[test]
fn test_index_api_request_roundtrip() {
    let json = r#"{
        "data": {
            "chat": {"id":"c1","name":"General","sn":"sn","type":"group"},
            "overlap_messages": [],
            "new_messages": [
                {"id":"m1","time":1,"text":"x","sender_id":"u","is_system":false,"is_hidden":false,"is_forward":false,"is_quote":false}
            ]
        }
    }"#;
    let req: IndexAPIRequest = serde_json::from_str(json).expect("parse");
    assert_eq!(req.data.new_messages.len(), 1);
    let reser = serde_json::to_string(&req).unwrap();
    let _again: IndexAPIRequest = serde_json::from_str(&reser).expect("roundtrip");
}
