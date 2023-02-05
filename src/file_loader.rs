use std::fs;
use serde::{Deserialize, Serialize};
use crate::utils::get_bool;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerData {
    pub name: String,
    pub prays: u32,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            name: String::from("Player"),
            prays: 0,
        }
    }
}

pub fn load_file() -> PlayerData {
    println!("load save-file? (y/n)");
    let should_load = get_bool().expect("should be a bool");

    if !should_load {
        return PlayerData::default();
    }

    let file_contents = fs::read_to_string("player_data.json")
        .expect("player_data.json should be readable");

    let player_data: PlayerData = serde_json::from_str(&file_contents)
        .expect("should be able to parse as json");

    player_data
}
