use serde::{Deserialize, Serialize};

use crate::enums::proficiencies::Proficiency;

#[derive(Serialize,Deserialize)]
pub struct WeaponProficiency {
    pub name: String,
    pub proficiency_level: Proficiency,
}

impl WeaponProficiency{
    pub fn default() -> [WeaponProficiency; 13] {
        [
        WeaponProficiency {name: "Blade".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Hammer".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Pistol".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Rifle".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Sniper".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Nueral".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Bow".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Explosive".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Thrown".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Shotgun".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Arc".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Gatling".to_string(), proficiency_level: Proficiency::Untrained},
        WeaponProficiency {name: "Whip".to_string(), proficiency_level: Proficiency::Untrained}
]
    }
    // pub fn from_json(file_path: &str) -> Vec<WeaponProficiency> {

    //     match fs::read_to_string(file_path) {
    //         Ok(file_content) => {
    //             match serde_json::from_str(&file_content) {
    //                 Ok(parsed) => parsed,
    //                 Err(e) => {
    //                     eprintln!("Error Deserialsing JSON: {}", e);
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