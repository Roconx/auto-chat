use serde::{Serialize, Deserialize};

const PLAYER_NAME_ADRESS: &str = "https://127.0.0.1:2999/liveclientdata/activeplayername";

const PLAYER_STATS_ADRESS: &str = "https://127.0.0.1:2999/liveclientdata/playerscores?summonerName=";

pub async fn get_name() -> Result<String, reqwest::Error> {
    // Requests the league api for player name, ignores certificate error
    let response: String = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .get(PLAYER_NAME_ADRESS)
            .send()
            .await?
            .text()
            .await?;

    // Removes " from the name 
    let player_name = response.replace("\"", "");

    // Returns the player's name
    Ok(player_name)
}
