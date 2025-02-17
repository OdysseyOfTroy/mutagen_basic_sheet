use eframe::{self, egui::{self, Context}};

#[derive(Default)]
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

pub struct CharacterApp {
    character: Character
}

impl Default for CharacterApp {
    fn default() -> Self {
        Self { character: Character::default(),
        }
    }
}

impl eframe::App for CharacterApp{
    fn update(&mut self, ctx:&Context , _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mutagen Character Sheet");
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.character.name);
            ui.label("Class:");
            ui.text_edit_singleline(&mut self.character.class);
            ui.label("Threat Level:");
            ui.add(egui::Slider::new(&mut self.character.threat, 1..=100));
            ui.label("Strength:");
            ui.add(egui::Slider::new(&mut self.character.strength, 1..=100));
            ui.label("Discipline:");
            ui.add(egui::Slider::new(&mut self.character.discipline, 1..=100));
            ui.label("Constitution:");
            ui.add(egui::Slider::new(&mut self.character.constitution, 1..=100));
            ui.label("Intelligence:");
            ui.add(egui::Slider::new(&mut self.character.intelligence, 1..=100));
            ui.label("Sense:");
            ui.add(egui::Slider::new(&mut self.character.sense, 1..=100));
            ui.label("Will:");
            ui.add(egui::Slider::new(&mut self.character.will, 1..=100));
        });
    }
}