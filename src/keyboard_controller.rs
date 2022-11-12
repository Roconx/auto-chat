use enigo::{Enigo, Key, KeyboardControllable};
use crate::parser;

pub enum Action<'a> {
    Message(&'a Vec<String>),
    Mastery,
    Surrender,
}

pub fn send_message(message: &str) {
    // Sends a message to chat
    let mut enigo = Enigo::new();

    enigo.key_click(Key::Return);

    enigo.key_sequence(message);

    enigo.key_click(Key::Return);
}

pub fn perform_action(action: &Action) {
    match action {
        Action::Message(file) => {
            let message = parser::get_random(&file);
            send_message(&message);
        },
        Action::Mastery => send_message("/masterybadge"),
        Action::Surrender => send_message("/surrender"),
    };
}
