use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Clone, PartialEq, Copy)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum Proficiency {
    #[default]
    Untrained,
    Proficient,
    Expert,
    Master
}


impl Proficiency {
    pub fn iterator() -> impl Iterator<Item = Proficiency> {
        [Proficiency::Untrained, Proficiency::Proficient, Proficiency::Expert, Proficiency::Master].iter().copied()
    }

    pub fn value(&self) -> u8 {
        match self {
            Proficiency::Untrained => 0,
            Proficiency::Proficient => 10,
            Proficiency::Expert => 20,
            Proficiency::Master => 30,
        }
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
