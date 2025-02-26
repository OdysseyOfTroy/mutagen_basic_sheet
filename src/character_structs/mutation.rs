use serde::{Deserialize, Serialize};

use crate::enums::traits::Traits;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Mutation {
    pub name: String,
    pub main_trait: Traits,
}

impl Mutation {
    pub fn default() -> Vec<Mutation> {
        [
            Mutation {name:"Nova".to_string(), main_trait:Traits::Will},
            Mutation {name:"Biohazard".to_string(), main_trait:Traits::Constitution},
            Mutation {name: "Chimera".to_string(), main_trait: Traits::Strength},
            Mutation {name: "Terra".to_string(), main_trait: Traits::Constitution},
            Mutation {name: "Spectre".to_string(), main_trait: Traits::Discipline},
            Mutation {name: "Prophet".to_string(), main_trait: Traits::Sense},
        ].to_vec()
    }
}
