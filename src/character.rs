use std::fmt;

pub struct Character {
    name: String,
    class: String,
    threat: u8,
    strength: u8,
    discipline: u8,
    constitution: u8,
    intelligence: u8,
    sense: u8,
    will: u8,
}

impl Character {
    pub fn new(
        name: &str,
        class: &str,
        threat: u8,
        strength: u8,
        discipline: u8,
        constitution: u8,
        intelligence: u8,
        sense: u8,
        will: u8,
    ) -> Self {
        Self {
            name: name.to_string(),
            class: class.to_string(),
            threat,
            strength,
            discipline,
            constitution,
            intelligence,
            sense,
            will,
        }
    }

    pub fn display(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Character Sheet:\nName: {}\nclass: {}\nthreat: {}\nstrength: {}\ndiscipline: {}\nconstitution: {}\nintelligence: {}\nsense: {}\nwill: {}",
            self.name,
            self.class,
            self.threat,
            self.strength,
            self.discipline,
            self.constitution,
            self.intelligence,
            self.sense,
            self.will,
        )
    }
}