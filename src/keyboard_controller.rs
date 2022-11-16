use crate::parser;
use enigo::{Enigo, KeyboardControllable};
use tfc::{traits::*, Context};

pub enum Action<'a> {
    Message(&'a Vec<String>),
    Mastery,
    Surrender,
    None,
}

pub fn send_message(message: &str) {
    // Sends a message to chat
    let mut enigo = Enigo::new();
    let mut ctx = Context::new().unwrap();

    enigo.key_click(enigo::Key::Return);

    ctx.unicode_string(message).unwrap();

    enigo.key_click(enigo::Key::Return);
}

pub fn perform_action(action: &Action) {
    match action {
        Action::Message(file) => {
            let message = parser::get_random(file);
            send_message(&message);
        }
        Action::Mastery => send_message("/masterybadge"),
        Action::Surrender => send_message("/ff"),
        Action::None => (),
    };
}
