use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize,Deserialize)]

pub enum Traits {
    Strength,
    Discipline,
    Constitution,
    Intelligence,
    Sense,
    Will
}

impl Traits {
    pub fn iterator() -> impl Iterator<Item = Traits> {
        [Traits::Strength, Traits::Discipline, Traits::Constitution, Traits::Intelligence, Traits::Sense, Traits::Will].iter().copied()
    }
}

impl fmt::Display for Traits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Traits::Strength => write!(f, "Strength"),
            Traits::Discipline => write!(f, "Discipline"),
            Traits::Constitution => write!(f, "Constitution"),
            Traits::Intelligence => write!(f, "Intelligence"),
            Traits::Sense => write!(f, "Sense"),
            Traits::Will => write!(f, "Will")
        }
    }
}