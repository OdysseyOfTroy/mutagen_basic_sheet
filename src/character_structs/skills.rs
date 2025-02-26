use serde::{Deserialize, Serialize};

use crate::enums::proficiencies::Proficiency;

#[derive(Serialize,Deserialize, Clone)]
pub struct Skill {
    pub name: String,
    pub proficiency_level: Proficiency,
}



impl Skill {
    pub fn default() -> Vec<Skill>{
        [
        Skill {name: "Academia".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Athletics".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Deception".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Endurance".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Force".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Insight".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Intimidation".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Piloting".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Investigation".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Medicine".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Perception".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Persuasion".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Precision".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Resistance".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Science".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Stealth".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Survival".to_string(), proficiency_level: Proficiency::Untrained},
        Skill {name: "Tech".to_string(), proficiency_level: Proficiency::Untrained}
    ].to_vec()

    }
    // pub fn from_json(file_path: &str) -> Vec<Skill>{

    //     match fs::read_to_string(file_path) {
    //         Ok(file_content) => { 
    //             match serde_json::from_str(&file_content){
    //                 Ok(parsed) => parsed,
    //                 Err(e) => {
    //                     eprintln!("Error Deserialising JSON: {}", e);
    //                     Vec::new()
    //                 }
    //             }
    //         }
    //         Err(e) => { eprintln!("Error reading file {}: {}", file_path, e);
    //         Vec::new()
    //     },
    //     }
    // }
}