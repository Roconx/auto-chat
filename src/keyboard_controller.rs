use crate::parser;
use enigo::{Enigo, KeyboardControllable};
use tfc::{traits::*, Context, Key};
use std::{thread, time};

pub enum Action<'a> {
    Message(&'a Vec<String>),
    Mastery,
    Surrender,
}

pub fn send_message(message: &str) {
    // Sends a message to chat
    let mut enigo = Enigo::new();
    let mut ctx = Context::new().unwrap();
    let delay = time::Duration::from_millis(100);

    enigo.key_click(enigo::Key::Return);

    ctx.unicode_string(message).unwrap();

    thread::sleep(delay);

    enigo.key_click(enigo::Key::Return);
}

pub fn press_key(key: Key) {
    // Presses the key
    let mut ctx = Context::new().unwrap();

    ctx.key_click(Key::Period).unwrap();

    println!("Death detected");
}

pub fn perform_action(action: &Action) {
    match action {
        Action::Message(file) => {
            let message = parser::get_random(file);
            send_message(&message);
        },
        Action::Mastery => send_message("/masterybadge"),
        Action::Surrender => send_message("/ff"),
    };
}
