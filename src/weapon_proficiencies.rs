use std::fs;
use serde::{Deserialize, Serialize};

use crate::enums::Proficiency;

#[derive(Serialize,Deserialize, Clone)]
pub struct WeaponProficiency {
    pub name: String,
    pub proficiency_level: Proficiency,
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
    pub fn from_json(file_path: &str) -> Self {

        match fs::read_to_string(file_path) {
            Ok(file_content) => {
                match serde_json::from_str(&file_content) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        eprintln!("Error Deserialsing JSON: {}", e);
                        Self::default()
                    }
                }
            }
            Err(e) => { eprintln!("Error reading file {}: {}", file_path, e);
            Self::default()
        },
        }
    }
}