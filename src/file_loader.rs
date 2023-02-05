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
