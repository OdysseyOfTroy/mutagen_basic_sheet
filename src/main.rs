mod character;
mod mutation;

use std::path::PathBuf;

use character::Character;
use eframe::egui::{self, Label};
use egui_file_dialog::FileDialog;
use mutation::Mutations;

struct CharacterApp {
    character: Character,
    mutations: Mutations,
    selected_mutation_trait: String,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
}

impl CharacterApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        let character = Character::from_json("character.json").unwrap_or_default();
        let mutations = Mutations::from_json("src/base_data/classes.json").unwrap_or_default();
        let selected_mutation_trait = String::new();
        if mutations.options.is_empty() {println!("options is empty")}
        Self { character, mutations, selected_mutation_trait, file_dialog: FileDialog::new(), picked_file: None}
    }
}

impl eframe::App for CharacterApp{
    fn update(&mut self, ctx: &egui::Context , _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            if ui.button("Pick file").clicked() {
                // Open the file dialog to pick a file.
                self.file_dialog.pick_file();
            }

            ui.label(format!("Picked file: {:?}", self.picked_file));

            // Update the dialog
            self.file_dialog.update(ctx);

            // Check if the user picked a file.
            if let Some(path) = self.file_dialog.take_picked() {
                self.picked_file = Some(path.to_path_buf());
            }
        });
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
        });}
}
fn main() -> eframe::Result<()> {
    eframe::run_native("Mutagen Character Creature", eframe::NativeOptions::default(),
     Box::new(|ctx| Ok(Box::new(CharacterApp::new(ctx)))))
}
