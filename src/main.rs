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
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>, 
    save_dialog: FileDialog
}

impl CharacterApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        let character = Character::from_json("character.json").unwrap_or_default();
        let mutations = Mutations::from_json("src/base_data/classes.json").unwrap_or_default();
        if mutations.options.is_empty() {println!("options is empty")}
        Self { character, mutations, file_dialog: FileDialog::new(), picked_file: None, save_dialog: FileDialog::new()}
    }
}

impl eframe::App for CharacterApp{
    fn update(&mut self, ctx: &egui::Context , _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            if ui.button("Pick file").clicked() {
                // Open the file dialog to pick a file.
                self.file_dialog.pick_file();
            }

            if ui.button("Save").clicked() {
                self.save_dialog.save_file();
            }

            ui.label(format!("Picked file: {:?}", self.picked_file));

            // Update the dialog
            self.file_dialog.update(ctx);

            // Check if the user picked a file.
            if let Some(path) = self.file_dialog.take_picked() {
                self.picked_file = Some(path.to_path_buf());
                let character_path_str = path.to_string_lossy().to_string();
                self.character = Character::from_json(&character_path_str).unwrap_or_default();
            }

            

            if let Some(path) = self.save_dialog.update(ctx).picked() {
                let saving_path_str = path.to_string_lossy().to_string();
                println!("Mutation data before saving {:?}", self.character.mutation);
                if let Err(err) = self.character.to_json(&saving_path_str) {
                    eprintln!("Failed to save character: {}", err);
                }};
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mutagen Character Sheet");
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.character.name);
            egui::ComboBox::from_label("Mutation")
                .selected_text(&self.character.mutation.name)
                .show_ui(ui, |ui|{
                    for mutation in &self.mutations.options {
                        if ui.selectable_value(&mut self.character.mutation, mutation.clone(), &mutation.name).clicked() {
                            
                        }
                    }
                }); 
            ui.label("main trait:");
            ui.add(Label::new(&self.character.mutation.main_trait));
            ui.label("Threat Level:");
            ui.add(egui::Slider::new(&mut self.character.threat, 1..=10));
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
