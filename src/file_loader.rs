/*
 * MIT License
 *
 * Copyright (c) 2023 rad
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::fs;
use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub enum SaveFileError {
    IoError(std::io::Error),
    JsonError(serde_json::error::Error),
}

// returns a result of either a PlayerData struct or a file not found error
pub fn load_save_file(path: &str) -> Result<PlayerData, SaveFileError>{

    let file_contents = match fs::read_to_string(path) {
        Ok(file_contents) => file_contents,
        Err(e) => return Err(SaveFileError::IoError(e)),
    };

    let player_data: PlayerData = match serde_json::from_str(&file_contents) {
        Ok(player_data) => player_data,
        Err(e) => return Err(SaveFileError::JsonError(e)),
    };

    Ok(player_data)
}

// saves a PlayerData struct to a file at the specified path
pub fn save_save_file(path: &str, player_data: &PlayerData) -> Result<(), SaveFileError> {

    let player_data_json = match serde_json::to_string(&player_data) {
        Ok(player_data_json) => player_data_json,
        Err(e) => return Err(SaveFileError::JsonError(e)),
    };

    match fs::write(path, player_data_json) {
        Ok(_) => Ok(()),
        Err(e) => Err(SaveFileError::IoError(e)),
    }

}
