mod client;
use std::{thread, time};

#[tokio::main]
async fn main() {
    // Gets the name of the player
    let name = loop {
        match client::get_name().await {
            Ok(name) => break name,
            Err(e) => println!("Client not found: {}", e),
        }
        // Sleeps 5 seconds
        let five_seconds = time::Duration::from_secs(5);
        thread::sleep(five_seconds);
    };

    // Gets the player's score
    loop {
        match client::get_score(name.as_str()).await {
            Ok(score) => {
                println!("Score: {:#?}", score);
            },
            Err(e) => println!("Error checking score: {}", e),
        }

        let five_seconds = time::Duration::from_secs(5);
        thread::sleep(five_seconds);
    }
}
