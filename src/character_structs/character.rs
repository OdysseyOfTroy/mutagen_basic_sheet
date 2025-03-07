use std::{fs::{self, File}, io::Write};

use serde::{Deserialize, Serialize};

use crate::enums::traits::Traits;
use crate::character_structs::{skills::Skill, weapon_proficiencies::WeaponProficiency, mutation::Mutation};

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
    pub weapon_proficiencies: [WeaponProficiency; 13]
}

impl Default for Character {
    fn default() -> Self {
        Character { 
            name: "".to_string(), 
            mutation: Mutation { name:"Chimera".to_string(), main_trait: Traits::Strength },
            threat: 1, 
            strength: 20, 
            discipline: 20, 
            constitution: 20, 
            intelligence: 20, 
            sense: 20, 
            will: 20,
            skills: Skill::default(),
            weapon_proficiencies: WeaponProficiency::default(),
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
        match character.mutation.main_trait {
            Traits::Strength => character.strength,
            Traits::Discipline => character.discipline,
            Traits::Constitution => character.constitution,
            Traits::Intelligence => character.intelligence,
            Traits::Sense => character.sense,
            Traits::Will => character.will,
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

    pub fn calculate_success(trait_value: u8, prof_value: u8, misc_value: i8) -> u8 {
        //pot_succ previously declared as i8 but was changed to i16 to prevent overflow as pot succ has the potential range of -30 to 180 exceeding the range of i8
        let pot_succ: i16 = trait_value as i16 + prof_value as i16 + misc_value as i16;
        if pot_succ < 1 {
            return 1;
        } else {
            return pot_succ as u8;
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
        //pot_crit_fail previously declared as u8 but was changed to u16 to prevent overflow as cam_value can be up to 100 so the max value of pot_crit_fail can be 200
        let pot_crit_fail: u16 = cam_value as u16 * 2;
        if pot_crit_fail > 100 {
            return 100;
        } else {
            return pot_crit_fail as u8;
        }
    }
}