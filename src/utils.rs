use user_input::str;

pub const COPYRIGHT_TEXT: &str = "(c) Copyright NotThatRqd 2023 All Rights Reserved";

// TODO: use thiserror
#[derive(Debug)]
pub struct NotABoolError;

pub fn get_bool() -> Result<bool, NotABoolError> {
    let res = str();
    if res == "y" {
        Ok(true)
    } else if res == "n" {
        Ok(false)
    } else {
        Err(NotABoolError)
    }
}
