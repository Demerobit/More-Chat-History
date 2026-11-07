fn restore(value: (), chat: &mut Chat) {
    let _ = value;
    more_chat_history::restore(chat);
}
