mod client;
mod keyboard_controller;
mod parser;
use crate::client::{Changed, Score};
use crate::keyboard_controller::Action;
use std::{thread, time};

#[tokio::main]
async fn main() {
    // Loads files
    let files = parser::read_files();

    let death = Action::Message(&files[0]);

    let kill = Action::Mastery;

    // Gets the name of the player
    let mut name = get_name().await;
    println!("Client detected!");

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
                }
                // Replaces current score with the new score
                current_score = score;
            }
            Err(e) => {
                println!("Error checking score: {}", e);
                name = get_name().await;
            },
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
            Err(e) => println!("Client not found: {}", e),
        }
        // Waits 5 seconds and tries again
        let five_seconds = time::Duration::from_secs(5);
        thread::sleep(five_seconds);
    }
}
