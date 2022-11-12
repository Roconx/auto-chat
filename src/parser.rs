use rand::seq::SliceRandom;
use std::fs;

const FILES: [&str; 2] = ["you_died.txt", "you_killed.txt"];

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
