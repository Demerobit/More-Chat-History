// More Chat History runtime module
use std::cell::RefCell;

pub const MAX_CHAT_HISTORY: usize = 10_000;

pub struct PreState {
    pub received: u64,
    pub lines: Vec<super::Line>,
    pub messages: Vec<super::Message>,
}

thread_local! {
    static PRE_STATE: RefCell<Option<PreState>> = const { RefCell::new(None) };
}

pub fn stash(chat: &super::Chat) {
    PRE_STATE.with(|cell| {
        *cell.borrow_mut() = Some(PreState {
            received: chat.received,
            lines: chat.lines.clone(),
            messages: chat.messages.clone(),
        });
    });
}

pub fn restore(chat: &mut super::Chat) {
    PRE_STATE.with(|cell| {
        let Some(pre) = cell.borrow_mut().take() else {
            return;
        };

        // If the message was rejected (e.g. abilities.shows returned false),
        // chat.received did not increment, and vanilla did not insert or truncate.
        if chat.received == pre.received {
            return;
        }

        // Determine if compact repeat merge occurred
        let is_compact_repeat = chat.messages.len() == pre.messages.len()
            && !chat.messages.is_empty()
            && !pre.messages.is_empty()
            && chat.messages[0].repeats > pre.messages[0].repeats;

        if is_compact_repeat {
            let shown = pre.messages[0].wrapped.min(pre.lines.len());
            let new_lines = chat.messages[0].wrapped;
            let surviving_lines = chat.lines.len().saturating_sub(new_lines);
            let removed_start = shown + surviving_lines;
            if removed_start < pre.lines.len() {
                chat.lines.extend_from_slice(&pre.lines[removed_start..]);
                chat.lines.truncate(MAX_CHAT_HISTORY);
            }
            return;
        }

        // 1. Messages:
        // Vanilla inserts 1 message at index 0 and calls truncate(100).
        // Surviving older messages from pre.messages in chat.messages are at indices 1..chat.messages.len().
        let surviving_messages = chat.messages.len().saturating_sub(1);
        if surviving_messages < pre.messages.len() {
            // Vanilla truncated away the messages starting at index `surviving_messages`
            chat.messages.extend_from_slice(&pre.messages[surviving_messages..]);
            chat.messages.truncate(MAX_CHAT_HISTORY);
        }

        // 2. Lines:
        // Vanilla wrapped the message and prepended `new_lines` at index 0,
        // then called lines.truncate(100).
        let new_lines = chat.messages.first().map_or(0, |m| m.wrapped);
        let surviving_lines = chat.lines.len().saturating_sub(new_lines);
        if surviving_lines < pre.lines.len() {
            // Vanilla truncated away the lines starting at index `surviving_lines`
            chat.lines.extend_from_slice(&pre.lines[surviving_lines..]);
            chat.lines.truncate(MAX_CHAT_HISTORY);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{font::Font, hud::Rect, text::Text};
    use std::{collections::HashMap, path::Path};

    fn test_font() -> Font {
        Font::load(
            Path::new("/nonexistent-horizon-chat-test-resources"),
            &HashMap::new(),
            &[],
            (1, 1),
            (0, 0),
            Rect {
                u0: 0.0,
                v0: 0.0,
                u1: 1.0,
                v1: 1.0,
                w: 1,
                h: 1,
            },
        )
    }

    #[test]
    fn regression_crossing_90_and_100_messages_without_duplicates() {
        let font = test_font();
        let mut chat = super::super::Chat::default();

        // 1. Send 91 messages
        for i in 1..=91 {
            chat.add_message(&font, Text::literal(format!("msg_{i}")), None, super::super::ChatSource::Player, i, false);
        }
        assert_eq!(chat.messages.len(), 91);
        assert_eq!(chat.lines.len(), 91);

        // 2. Send 92nd message - crossing 90/91 boundary
        chat.add_message(&font, Text::literal("msg_92"), None, super::super::ChatSource::Player, 92, false);
        assert_eq!(chat.messages.len(), 92, "must have exactly 92 messages with no duplicates");
        assert_eq!(chat.lines.len(), 92, "must have exactly 92 lines with no duplicates");

        // Verify each message is unique
        let msg_texts: Vec<String> = chat.messages.iter().map(|m| m.content.plain().to_string()).collect();
        let mut unique_msgs = msg_texts.clone();
        unique_msgs.sort();
        unique_msgs.dedup();
        assert_eq!(msg_texts.len(), unique_msgs.len(), "messages must not have any duplicates");

        // 3. Send up to 150 messages - crossing 100 limit
        for i in 93..=150 {
            chat.add_message(&font, Text::literal(format!("msg_{i}")), None, super::super::ChatSource::Player, i, false);
        }
        assert_eq!(chat.messages.len(), 150, "messages must be preserved past 100 up to limit");
        assert_eq!(chat.lines.len(), 150, "lines must be preserved past 100 up to limit");

        // Check order: newest first
        assert_eq!(chat.messages[0].content.plain(), "msg_150");
        assert_eq!(chat.messages[149].content.plain(), "msg_1");
        assert_eq!(chat.lines[0].content.plain(), "msg_150");
        assert_eq!(chat.lines[149].content.plain(), "msg_1");
    }

    #[test]
    fn regression_wrapped_messages_crossing_limit() {
        let font = test_font();
        let mut chat = super::super::Chat::default();

        // A long message that wraps across multiple lines
        let long_line = "a ".repeat(100);
        for i in 1..=40 {
            chat.add_message(&font, Text::literal(format!("{i}: {long_line}")), None, super::super::ChatSource::Player, i, false);
        }

        // Each message wraps to > 1 line, so total lines will cross 100 well before 100 messages
        assert!(chat.lines.len() > 100, "lines should exceed 100");
        assert_eq!(chat.messages.len(), 40);

        // Verify no duplicates in lines
        let line_count = chat.lines.len();
        chat.add_message(&font, Text::literal(format!("41: {long_line}")), None, super::super::ChatSource::Player, 41, false);
        assert_eq!(chat.messages.len(), 41);
        let wrapped_added = chat.messages[0].wrapped;
        assert_eq!(chat.lines.len(), line_count + wrapped_added, "all wrapped lines must be preserved without truncation");
    }

    #[test]
    fn regression_rejected_messages_do_not_alter_history() {
        let font = test_font();
        let mut chat = super::super::Chat::default();

        // Prepopulate with 95 messages
        for i in 1..=95 {
            chat.add_message(&font, Text::literal(format!("msg_{i}")), None, super::super::ChatSource::Player, i, false);
        }
        assert_eq!(chat.messages.len(), 95);
        assert_eq!(chat.lines.len(), 95);

        // Set visibility to System only (rejects Player messages)
        chat.set_abilities(&font, super::super::ChatAbilities::new(super::super::ChatVisibility::System));

        // Attempt to send a Player message
        chat.add_message(&font, Text::literal("rejected_player_msg"), None, super::super::ChatSource::Player, 96, false);

        // History must NOT change
        assert_eq!(chat.messages.len(), 95, "rejected message must not increase count or duplicate tail");
        assert_eq!(chat.lines.len(), 95, "rejected message must not alter lines");
        assert_eq!(chat.messages[0].content.plain(), "msg_95");
    }
}
