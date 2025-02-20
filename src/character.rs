use std::{fs::{self, File}, io::Write};

use serde::{Deserialize, Serialize};

use crate::{enums::traits::Traits, mutation::Mutation, skills::Skill, weapon_proficiencies::WeaponProficiency};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub mutation: Mutation,
    pub threat: u8,
    pub strength: u8,
    pub discipline: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub sense: u8,
    pub will: u8,
    pub skills: Vec<Skill>,
    pub weapon_proficiencies: Vec<WeaponProficiency>
}

impl Default for Character {
    fn default() -> Self {
        Character { 
            name: "".to_string(), 
            mutation: Mutation { name:"Chimera".to_string(), main_trait: "strength".to_string() },
            threat: 1, 
            strength: 20, 
            discipline: 20, 
            constitution: 20, 
            intelligence: 20, 
            sense: 20, 
            will: 20,
            skills: Skill::from_json("src/base_data/skills.json") ,
            weapon_proficiencies: WeaponProficiency::from_json("src/base_data/weapon_proficiencies.json"),
         }
    }
}

impl Character {
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
    

    pub fn to_json(&self, file_path: &str) -> Result<(), std::io::Error>{
        let json = serde_json::to_string_pretty(self).expect("Failed to Serialise character");
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn calculate_mod(trait_value: u8) -> String {
        return ((trait_value / 10) - 2).to_string();
    }



    pub fn calculate_ability_strike_trait(character: &Character) -> u8 {
        match character.mutation.main_trait.to_lowercase().as_str() {
            "strength" => character.strength,
            "discipline" => character.discipline,
            "constitution" => character.constitution,
            "intelligence" => character.intelligence,
            "sense" => character.sense,
            "will" => character.will,
            _ => 0, // Default value if trait is not found
        }
    }

    pub fn get_trait_value(character: &Character, selected_trait: &Traits) -> u8 {
        match selected_trait.to_string().as_str() {
            "Strength" => character.strength,
            "Discipline" => character.discipline,
            "Constitution" => character.constitution,
            "Intelligence" => character.intelligence,
            "Sense" => character.sense,
            "Will" => character.will,
            _ => 0, // Default value if trait is not found
        }
    }
    
    pub fn calculate_crit_success(cam_value: u8) -> u8 {
        let pot_crit_succ: i8 = (cam_value / 4).try_into().unwrap();
        if pot_crit_succ < 1 {
            return 1;
        } else {
            return pot_crit_succ as u8;
        }
    }

    pub fn calculate_crit_fail(cam_value: u8) -> u8 {
        let pot_crit_fail: u8 = cam_value * 2;
        if pot_crit_fail > 100 {
            return 100;
        } else {
            return pot_crit_fail as u8;
        }
    }
}