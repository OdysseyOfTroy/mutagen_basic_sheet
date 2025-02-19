use std::fs;
use serde::{Deserialize, Serialize};

use crate::enums::Proficiency;



#[derive(Serialize,Deserialize, Clone)]
pub struct Skill {
    pub name: String,
    pub proficiency_level: Proficiency,
}

#[derive(Serialize,Deserialize)]
pub struct Skills {
    pub skills: Vec<Skill>
}

impl Default for Skills {
    fn default() -> Self {
        Skills { skills: [Skill {name: "Academia".to_owned(),proficiency_level: Proficiency::Untrained}].to_vec() }
    }
}

impl Skills {
    pub fn from_json(file_path: &str) -> Self{

        match fs::read_to_string(file_path) {
            Ok(file_content) => { 
                match serde_json::from_str(&file_content){
                    Ok(parsed) => parsed,
                    Err(e) => {
                        eprintln!("Error Deserialising JSON: {}", e);
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