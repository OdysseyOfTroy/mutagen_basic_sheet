use std::{fs::{self, File}, io::BufReader};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Mutation {
    pub name: String,
    pub main_trait: String,
}

#[derive(Serialize,Deserialize)]
pub struct Mutations {
    pub options: Vec<Mutation>
}

impl Default for Mutations {
    fn default() -> Self {
        Mutations {options:
        [Mutation { name: "BioHazard".to_owned(), main_trait: "will".to_owned() }].to_vec()
    }}
}

impl Mutations {
    pub fn from_json(file_path: &str) -> Result<Self, serde_json::Error> {
        match fs::read_to_string(file_path) {
            Ok(file_content) => {serde_json::from_str(&file_content)}
            Err(e) => { println!("Error parsing JSON: {}", e); Ok(Self::default())
        },
        }
    }
}