use std::{fs::{self, File}, io::Write};

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default)]
pub struct Character {
    pub name: String,
    pub mutation: String,
    pub threat: u8,
    pub strength: u8,
    pub discipline: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub sense: u8,
    pub will: u8,
}

impl Character {
    pub fn from_json(file_path: &str) -> Result<Self, serde_json::Error> {
        match fs::read_to_string(file_path) {
            Ok(file_content) => serde_json::from_str(&file_content),
            Err(_) => Ok(Self::default()),
        }
    }

    pub fn to_json(&self, file_path: &str) -> Result<(), std::io::Error>{
        let json = serde_json::to_string_pretty(self).expect("Failed to Serialise character");
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}