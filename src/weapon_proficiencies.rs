use std::fs;
use serde::{Deserialize, Serialize};

use crate::enums::Proficiency;

#[derive(Serialize,Deserialize, Clone)]
pub struct WeaponProficiency {
    pub name: String,
    proficiency_level: Proficiency,
}

#[derive(Serialize,Deserialize)]
pub struct WeaponProficiencies {
    pub weapon_proficiencies: Vec<WeaponProficiency>
}

impl Default for WeaponProficiencies {
    fn default() -> Self {
        WeaponProficiencies { weapon_proficiencies: [].to_vec() }
    }
}

impl WeaponProficiencies {
    pub fn from_json(file_path: &str) -> Result<Self, serde_json::Error> {
        match fs::read_to_string(file_path) {
            Ok(file_content) => {serde_json::from_str(&file_content)}
            Err(e) => { println!("Error parsing JSON: {}", e); Ok(Self::default())
        },
        }
    }
}