// More Chat History runtime module
use std::cell::RefCell;

thread_local! {
    static STASHED_LINES: RefCell<Vec<super::Line>> = const { RefCell::new(Vec::new()) };
    static STASHED_MESSAGES: RefCell<Vec<super::Message>> = const { RefCell::new(Vec::new()) };
}

pub fn stash(chat: &super::Chat) {
    STASHED_LINES.with(|cell| {
        if chat.lines.len() >= 90 {
            *cell.borrow_mut() = chat.lines[90..].to_vec();
        } else {
            cell.borrow_mut().clear();
        }
    });
    STASHED_MESSAGES.with(|cell| {
        if chat.messages.len() >= 90 {
            *cell.borrow_mut() = chat.messages[90..].to_vec();
        } else {
            cell.borrow_mut().clear();
        }
    });
}

pub fn restore(chat: &mut super::Chat) {
    STASHED_LINES.with(|cell| {
        let mut stashed = cell.borrow_mut();
        if !stashed.is_empty() {
            // Keep existing lines (which are the newly prepended ones)
            // and append the stashed older lines at the end.
            chat.lines.append(&mut stashed);
            chat.lines.truncate(10_000);
        }
    });
    STASHED_MESSAGES.with(|cell| {
        let mut stashed = cell.borrow_mut();
        if !stashed.is_empty() {
            chat.messages.append(&mut stashed);
            chat.messages.truncate(10_000);
        }
    });
}
