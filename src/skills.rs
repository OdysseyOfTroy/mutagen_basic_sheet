use std::fs;
use serde::{Deserialize, Serialize};

use crate::enums::Proficiency;



#[derive(Serialize,Deserialize, Clone)]
pub struct Skill {
    pub name: String,
    proficiency_level: Proficiency,
}

#[derive(Serialize,Deserialize)]
pub struct Skills {
    pub skills: Vec<Skill>
}

impl Default for Skills {
    fn default() -> Self {
        Skills { skills: [].to_vec() }
    }
}

impl Skills {
    pub fn from_json(file_path: &str) -> Result<Self, serde_json::Error> {
        match fs::read_to_string(file_path) {
            Ok(file_content) => {serde_json::from_str(&file_content)}
            Err(e) => { println!("Error parsing JSON: {}", e); Ok(Self::default())
        },
        }
    }
}