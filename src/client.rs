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

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    assists: i16,
    #[serde(rename = "creepScore")]
    creep_score: i16,
    deaths: i16,
    kills: i16,
    #[serde(rename = "wardScore")]
    ward_score: f32,
}

pub async fn get_score(name: &str) -> Result<Score, reqwest::Error> {
    // Requests the league api for the player's score, ignores certificate error
    let score: Score = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .get(PLAYER_STATS_ADRESS.to_string() + name)
            .send()
            .await?
            .json()
            .await?;

    // Returns the score converted to Score struct
    Ok(score)
}
