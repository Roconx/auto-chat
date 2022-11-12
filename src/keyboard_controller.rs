use crate::parser;
use enigo::{Enigo, KeyboardControllable};
use tfc::{traits::*, Context, Key};

pub enum Action<'a> {
    Message(&'a Vec<String>),
    Mastery,
    Surrender,
}

pub fn send_message(message: &str) {
    // Sends a message to chat
    let mut enigo = Enigo::new();

    enigo.key_click(enigo::Key::Return);

    enigo.key_sequence(message);

    enigo.key_click(enigo::Key::Return);
}

pub fn press_key(key: Key) {
    // Presses the key
    let mut ctx = Context::new().unwrap();

    ctx.key_click(key).unwrap();
}

pub fn perform_action(action: &Action) {
    match action {
        Action::Message(file) => {
            let message = parser::get_random(&file);
            send_message(&message);
        }
        Action::Mastery => press_key(Key::Period),
        Action::Surrender => send_message("/surrender"),
    };
}
