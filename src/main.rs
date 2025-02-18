mod character;
mod mutation;


use character::Character;
use eframe::{egui::{self, Context, Label}, CreationContext};
use mutation::Mutations;

struct CharacterApp {
    character: Character,
    mutations: Mutations,
    selected_mutation_trait: String,
}

impl Default for CharacterApp {
    fn default() -> Self {
        let character = Character::from_json("character.json").unwrap_or_default();
        let mutations = Mutations::from_json("src/base_data/classes.json").unwrap_or_default();
        let selected_mutation_trait = String::new();
        if mutations.options.is_empty() {println!("options is empty")}
        Self { character, mutations, selected_mutation_trait }
    }
}

impl eframe::App for CharacterApp{
    fn update(&mut self, ctx:&Context , _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mutagen Character Sheet");
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.character.name);
            egui::ComboBox::from_label("Mutation")
                .selected_text(&self.character.mutation)
                .show_ui(ui, |ui|{
                    for mutation in &self.mutations.options {
                        if ui.selectable_value(&mut self.character.mutation, mutation.name.clone(), &mutation.name).clicked() {
                            self.selected_mutation_trait = mutation.main_trait.clone();
                        }
                    }
                });
            ui.label("main trait:");
            ui.add(Label::new(&self.selected_mutation_trait));
            // ui.label("Class:");
            // ui.text_edit_singleline(&mut self.character.class);
            ui.label("Threat Level:");
            ui.add(egui::Slider::new(&mut self.character.threat, 1..=20));
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
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Mutagen Character Creature", options, Box::new(|_cc: &CreationContext| Ok(Box::new(CharacterApp::default()))))
}
