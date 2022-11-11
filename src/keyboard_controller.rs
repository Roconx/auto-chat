use enigo::{Enigo, Key, KeyboardControllable};
use std::thread;
use std::time::Duration;

pub fn send_message(message: &str) {
    let one_second = Duration::from_secs(1);
    thread::sleep(one_second);

    let message = format!("/all {}", message);

    let mut enigo = Enigo::new();

    enigo.key_click(Key::Return);

    enigo.key_sequence(message.as_str());

    enigo.key_click(Key::Return);
}
