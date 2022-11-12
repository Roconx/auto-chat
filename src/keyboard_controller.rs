use enigo::{Enigo, Key, KeyboardControllable};

pub fn send_message(message: &str) {

    let message = format!("/all {}", message);

    let mut enigo = Enigo::new();

    enigo.key_click(Key::Return);

    enigo.key_sequence(message.as_str());

    enigo.key_click(Key::Return);
}
