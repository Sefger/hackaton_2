use shared::{Chat, ChatType, IndexAPIDataItem, IndexAPIResultItem, Message};
use std::collections::HashMap;

fn chat_type_label(t: ChatType) -> &'static str {
    match t {
        ChatType::Private => "private",
        ChatType::Group => "group",
        ChatType::Channel => "channel",
    }
}

fn is_indexable(m: &Message) -> bool {
    !m.is_hidden && !m.is_system
}

fn forward_quote_prefix(m: &Message) -> &'static str {
    if m.is_forward { "[forward] " } else if m.is_quote { "[quote] " } else { "" }
}

fn build_thread_index<'a>(overlap: &'a [Message]) -> HashMap<&'a str, &'a Message> {
    let mut map = HashMap::new();
    for m in overlap {
        if let Some(sn) = m.thread_sn.as_deref() {
            map.entry(sn).or_insert(m);
        }
    }
    map
}

fn build_page_content(m: &Message) -> String {
    format!("{}: {}", m.sender_id, m.text.trim())
}

fn build_dense_content(chat: &Chat, m: &Message, thread_parent: Option<&Message>) -> String {
    let mut out = String::new();
    out.push_str(&format!("Чат: {} [{}]\n", chat.name, chat_type_label(chat.chat_type)));
    if let Some(parent) = thread_parent {
        out.push_str(&format!("Тема: {}\n", parent.text.trim()));
    }
    out.push_str(&format!("От: {}\n", m.sender_id));
    out.push_str(forward_quote_prefix(m));
    out.push_str(m.text.trim());
    if !m.file_snippets.trim().is_empty() {
        out.push('\n');
        out.push_str(m.file_snippets.trim());
    }
    out
}

fn build_sparse_content(m: &Message) -> String {
    let mut parts: Vec<String> = Vec::new();
    parts.push(m.text.trim().to_string());
    if !m.file_snippets.trim().is_empty() {
        parts.push(m.file_snippets.trim().to_string());
    }
    if !m.mentions.is_empty() {
        parts.push(m.mentions.join(" "));
    }
    parts.join(" ")
}

pub fn chunk(data: &IndexAPIDataItem) -> Vec<IndexAPIResultItem> {
    let thread_idx = build_thread_index(&data.overlap_messages);
    data.new_messages
        .iter()
        .filter(|m| is_indexable(m))
        .map(|m| {
            let parent = m.thread_sn.as_deref().and_then(|sn| thread_idx.get(sn).copied());
            IndexAPIResultItem {
                page_content: build_page_content(m),
                dense_content: build_dense_content(&data.chat, m, parent),
                sparse_content: build_sparse_content(m),
                message_ids: vec![m.id.clone()],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_msg(id: &str, text: &str, sender: &str) -> Message {
        Message {
            id: id.into(),
            thread_sn: None,
            time: 0,
            text: text.into(),
            sender_id: sender.into(),
            file_snippets: String::new(),
            parts: vec![],
            mentions: vec![],
            member_event: None,
            is_system: false,
            is_hidden: false,
            is_forward: false,
            is_quote: false,
        }
    }

    fn mk_chat() -> Chat {
        Chat {
            id: "c".into(),
            name: "General".into(),
            sn: "gen".into(),
            chat_type: ChatType::Group,
            is_public: None,
            members_count: None,
            members: vec![],
        }
    }

    #[test]
    fn one_chunk_per_new_message() {
        let data = IndexAPIDataItem {
            chat: mk_chat(),
            overlap_messages: vec![],
            new_messages: vec![
                mk_msg("m1", "hello", "u1"),
                mk_msg("m2", "world", "u2"),
            ],
        };
        let chunks = chunk(&data);
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].message_ids, vec!["m1"]);
        assert_eq!(chunks[1].message_ids, vec!["m2"]);
    }

    #[test]
    fn skips_system_and_hidden() {
        let mut sys = mk_msg("ms", "sys", "u");
        sys.is_system = true;
        let mut hid = mk_msg("mh", "hid", "u");
        hid.is_hidden = true;
        let data = IndexAPIDataItem {
            chat: mk_chat(),
            overlap_messages: vec![],
            new_messages: vec![sys, hid, mk_msg("ok", "ok", "u")],
        };
        let chunks = chunk(&data);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].message_ids, vec!["ok"]);
    }

    #[test]
    fn page_content_contains_text_and_sender() {
        let data = IndexAPIDataItem {
            chat: mk_chat(),
            overlap_messages: vec![],
            new_messages: vec![mk_msg("m1", "ping", "alice")],
        };
        let c = chunk(&data);
        assert!(c[0].page_content.contains("alice"));
        assert!(c[0].page_content.contains("ping"));
    }

    #[test]
    fn dense_content_includes_chat_metadata() {
        let data = IndexAPIDataItem {
            chat: mk_chat(),
            overlap_messages: vec![],
            new_messages: vec![mk_msg("m1", "ping", "alice")],
        };
        let c = chunk(&data);
        assert!(c[0].dense_content.contains("General"));
        assert!(c[0].dense_content.contains("group"));
        assert!(c[0].dense_content.contains("alice"));
        assert!(c[0].dense_content.contains("ping"));
    }

    #[test]
    fn forward_and_quote_prefixes() {
        let mut fwd = mk_msg("f", "body", "u");
        fwd.is_forward = true;
        let mut quo = mk_msg("q", "body", "u");
        quo.is_quote = true;
        let data = IndexAPIDataItem {
            chat: mk_chat(),
            overlap_messages: vec![],
            new_messages: vec![fwd, quo],
        };
        let c = chunk(&data);
        assert!(c[0].dense_content.contains("[forward]"));
        assert!(c[1].dense_content.contains("[quote]"));
    }

    #[test]
    fn thread_context_from_overlap() {
        let mut parent = mk_msg("p", "original", "owner");
        parent.thread_sn = Some("t1".into());
        let mut reply = mk_msg("r", "reply", "u");
        reply.thread_sn = Some("t1".into());
        let data = IndexAPIDataItem {
            chat: mk_chat(),
            overlap_messages: vec![parent],
            new_messages: vec![reply],
        };
        let c = chunk(&data);
        assert_eq!(c.len(), 1);
        assert!(c[0].dense_content.contains("Тема:"));
        assert!(c[0].dense_content.contains("original"));
    }

    #[test]
    fn sparse_content_includes_mentions_and_snippets() {
        let mut m = mk_msg("m", "текст", "user1");
        m.mentions = vec!["@bob".into(), "@carol".into()];
        m.file_snippets = "doc-42.pdf".into();
        let data = IndexAPIDataItem {
            chat: mk_chat(),
            overlap_messages: vec![],
            new_messages: vec![m],
        };
        let c = chunk(&data);
        assert!(c[0].sparse_content.contains("текст"));
        assert!(c[0].sparse_content.contains("@bob"));
        assert!(c[0].sparse_content.contains("doc-42.pdf"));
    }

    #[test]
    fn empty_input_yields_empty_output() {
        let data = IndexAPIDataItem {
            chat: mk_chat(),
            overlap_messages: vec![],
            new_messages: vec![],
        };
        assert_eq!(chunk(&data).len(), 0);
    }
}
