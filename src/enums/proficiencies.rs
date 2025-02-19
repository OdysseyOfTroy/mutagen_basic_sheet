use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Proficiency {
    Untrained,
    Proficient,
    Expert,
    Master
}

impl Default for Proficiency {
    fn default() -> Self {
        Proficiency::Untrained
    }
}

impl fmt::Display for Proficiency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Proficiency::Untrained => write!(f, "untrained"),
            Proficiency::Proficient => write!(f, "proficient"),
            Proficiency::Expert => write!(f, "expert"),
            Proficiency::Master => write!(f, "master"),
        }
    }
}