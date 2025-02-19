use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Clone)]
pub enum Proficiency {
    Untrained,
    Proficient,
    Expert,
    Master
}