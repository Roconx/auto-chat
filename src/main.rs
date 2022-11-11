mod client;
mod parser;
mod keyboard_controller;
use std::{thread, time};
use crate::client::{Score, Changed};
use rand::seq::SliceRandom;


#[tokio::main]
async fn main() {
    // Loads files
    let files = parser::read_files();

    // Gets the name of the player
    let name = get_name().await;
    println!("Client detected!");

    let mut current_score = Score::blank_score();
    // Gets the player's score
    loop {
        match client::get_score(name.as_str()).await {
            Ok(score) => {
                // Gets changes 
                let changes = score.compare(&current_score);
                for change in changes {
                    match change {
                        Changed::Deaths => {
                            // Selects random phrase from Deaths file
                            let phrase = files[0].choose(&mut rand::thread_rng()).unwrap();
                            keyboard_controller::send_message(phrase);
                        },
                        Changed::Kills => {
                            // Selects random phrase from Kills file
                            let phrase = files[1].choose(&mut rand::thread_rng()).unwrap();
                            keyboard_controller::send_message(phrase);
                        },
                    };
                }
                // Replaces current score with the new score
                current_score = score;
            },
            Err(e) => println!("Error checking score: {}", e),
        }

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
        // Sleeps 5 seconds
        let five_seconds = time::Duration::from_secs(5);
        thread::sleep(five_seconds);
    }
}

