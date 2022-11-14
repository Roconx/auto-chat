mod client;
mod keyboard_controller;
mod parser;
use crate::client::{Changed, Score};
use std::io::{stdout, Write};
use std::{thread, time};

#[tokio::main]
async fn main() {
    // Loads files
    let files = parser::read_files();

    let config: parser::Config = parser::parse_config();

    let death = parser::str_to_action(config.death.as_str(), &files[0]);

    let kill = parser::str_to_action(config.kill.as_str(), &files[1]);

    // Gets the name of the player
    let mut name = get_name().await;
    print("Client detected!");

    let mut current_score = Score::blank_score();
    // Gets the player's score
    loop {
        match client::get_score(name.as_str()).await {
            Ok(score) => {
                // Gets changes
                let changes = score.compare(&current_score);
                // Iterates and handles all changes
                for change in changes {
                    match change {
                        Changed::Deaths => keyboard_controller::perform_action(&death),
                        Changed::Kills => keyboard_controller::perform_action(&kill),
                    };
                    let one_second = time::Duration::from_secs(1);
                    thread::sleep(one_second);
                }
                // Replaces current score with the new score
                current_score = score;
            }
            Err(_) => {
                print("Error checking score, waiting for score..");
                name = get_name().await;
            }
        }

        // Waits one second and tries again
        let one_second = time::Duration::from_secs(1);
        thread::sleep(one_second);
    }
}

async fn get_name() -> String {
    // Gets the name of the player
    loop {
        match client::get_name().await {
            Ok(name) => break name,
            Err(_) => {
                print("Client not found, waiting for client..");
            }
        }
        // Waits 5 seconds and tries again
        let five_seconds = time::Duration::from_secs(5);
        thread::sleep(five_seconds);
    }
}

fn print(message: &str) {
    // Clears the line and prints new message
    print!("\r                                        ");
    print!("\r{}", message);
    stdout().flush().unwrap();
}
