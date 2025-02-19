use std::fs;
use serde::{Deserialize, Serialize};

use crate::enums::Proficiency;

#[derive(Serialize,Deserialize, Clone)]
pub struct Skill {
    pub name: String,
    pub proficiency_level: Proficiency,
}

impl Skill {
    pub fn from_json(file_path: &str) -> Vec<Skill>{

        match fs::read_to_string(file_path) {
            Ok(file_content) => { 
                match serde_json::from_str(&file_content){
                    Ok(parsed) => parsed,
                    Err(e) => {
                        eprintln!("Error Deserialising JSON: {}", e);
                        Vec::new()
                    }
                }
            }
            Err(e) => { eprintln!("Error reading file {}: {}", file_path, e);
            Vec::new()
        },
        }
    }
}