use crate::keyboard_controller::Action;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs;

const FILES: [&str; 3] = ["death.txt", "kill.txt", "assist.txt"];
const CONFIG: &str = "config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub death: String,
    pub kill: String,
    pub assist: String,
}

fn read_file(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).expect("Couldn't read file");

    // Adds /all to the start of all phrases
    contents.lines().map(|s| format!("/all {}", s)).collect()
}

pub fn read_files() -> Vec<Vec<String>> {
    // Reads all files in FILES array
    let mut files: Vec<Vec<String>> = Vec::new();
    for i in 0..FILES.len() {
        let file = FILES[i];
        files.push(read_file(file))
    }
    files
}

pub fn get_random(phrases: &Vec<String>) -> String {
    // Chooses a random phrase from the vector
    let phrase = phrases.choose(&mut rand::thread_rng()).unwrap();
    phrase.to_string()
}

pub fn parse_config() -> Config {
    let contents = fs::read_to_string(CONFIG).unwrap();

    let json: Config = serde_json::from_str(&contents).expect("config.json format is not correct");

    json
}

pub fn str_to_action<'a>(str: &'a str, phrases: &'a Vec<String>) -> Action<'a> {
    match str {
        "message" => Action::Message(phrases),
        "mastery" => Action::Mastery,
        "surrender" => Action::Surrender,
        &_ => {
            panic!("Incorrect option in config.json, the avaliable options are: 'message', 'mastery', 'surrender'");
        }
    }
}
