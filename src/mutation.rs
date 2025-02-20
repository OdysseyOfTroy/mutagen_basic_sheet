use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Mutation {
    pub name: String,
    pub main_trait: String,
}

impl Mutation {
    pub fn default() -> Vec<Mutation> {
        [
            Mutation {name:"Nova".to_string(), main_trait:"will".to_string()},
            Mutation {name:"Biohazard".to_string(), main_trait:"constitution".to_string()},
            Mutation {name: "Chimera".to_string(), main_trait: "strength".to_string()},
            Mutation {name: "Terra".to_string(), main_trait: "constitution".to_string()},
            Mutation {name: "Spectre".to_string(), main_trait: "discipline".to_string()},
            Mutation {name: "Prophet".to_string(), main_trait: "sense".to_string()},
        ].to_vec()
    }
}
