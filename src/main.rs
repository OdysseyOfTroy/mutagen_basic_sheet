mod character;
mod mutation;

use std::path::PathBuf;

use character::Character;
use eframe::egui::{self, Label};
use egui_file_dialog::FileDialog;
use mutation::Mutations;

struct CharacterApp {
    character: Character,
    str_mod_text: String,
    dsc_mod_text: String,
    con_mod_text: String,
    int_mod_text: String,
    sns_mod_text: String,
    wil_mod_text: String,
    mutations: Mutations,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>, 
    save_dialog: FileDialog
}


impl CharacterApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        let character = Character::from_json("character.json").unwrap_or_default();
        let mutations = Mutations::from_json("src/base_data/classes.json").unwrap_or_default();

        let str_mod_text = Character::calculate_mod(character.strength);
        let dsc_mod_text = Character::calculate_mod(character.discipline);
        let con_mod_text = Character::calculate_mod(character.constitution);
        let int_mod_text = Character::calculate_mod(character.intelligence);
        let sns_mod_text = Character::calculate_mod(character.sense);
        let wil_mod_text = Character::calculate_mod(character.will);

        Self { character, mutations, file_dialog: FileDialog::new(), picked_file: None, save_dialog: FileDialog::new(),
            str_mod_text, dsc_mod_text, con_mod_text, int_mod_text, sns_mod_text, wil_mod_text }
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
                self.str_mod_text = Character::calculate_mod(self.character.strength);
                self.dsc_mod_text = Character::calculate_mod(self.character.discipline);
                self.con_mod_text = Character::calculate_mod(self.character.constitution);
                self.int_mod_text = Character::calculate_mod(self.character.intelligence);
                self.sns_mod_text = Character::calculate_mod(self.character.sense);
                self.wil_mod_text = Character::calculate_mod(self.character.will);
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
            if ui.add(egui::Slider::new(&mut self.character.strength, 1..=100)).changed() {
                let mut new_str_mod = ((self.character.strength as i8 / 10 as i8) - 2 as i8).to_string();
                change_trait_mod(&mut self.str_mod_text, &mut new_str_mod, ctx);
            };
            ui.label("Discipline:");
            if ui.add(egui::Slider::new(&mut self.character.discipline, 1..=100)).changed() {
                let mut new_dsc_mod = ((self.character.discipline as i8 / 10 as i8) - 2 as i8).to_string();
                change_trait_mod(&mut self.dsc_mod_text, &mut new_dsc_mod, ctx);
            };
            ui.label("Constitution:");
            if ui.add(egui::Slider::new(&mut self.character.constitution, 1..=100)).changed() {
                let mut new_con_mod = ((self.character.constitution as i8 / 10 as i8) - 2 as i8).to_string();
                change_trait_mod(&mut self.con_mod_text, &mut new_con_mod, ctx);
            };
            ui.label("Intelligence:");
            if ui.add(egui::Slider::new(&mut self.character.intelligence, 1..=100)).changed() {
                let mut new_int_mod = ((self.character.intelligence as i8 / 10 as i8) - 2 as i8).to_string();
                change_trait_mod(&mut self.int_mod_text, &mut new_int_mod, ctx);
            };
            ui.label("Sense:");
            if ui.add(egui::Slider::new(&mut self.character.sense, 1..=100)).changed() {
                let mut new_sns_mod = ((self.character.sense as i8 / 10 as i8) - 2 as i8).to_string();
                change_trait_mod(&mut self.sns_mod_text, &mut new_sns_mod, ctx);
            };
            ui.label("Will:");
            if ui.add(egui::Slider::new(&mut self.character.will, 1..=100)).changed() {
                let mut new_wil_mod = ((self.character.will as i8 / 10 as i8) - 2 as i8).to_string();
                change_trait_mod(&mut self.wil_mod_text, &mut new_wil_mod, ctx);
            };

            ui.add(egui::Label::new(&self.str_mod_text));
            ui.add(egui::Label::new(&self.dsc_mod_text));
            ui.add(egui::Label::new(&self.con_mod_text));
            ui.add(egui::Label::new(&self.int_mod_text));
            ui.add(egui::Label::new(&self.sns_mod_text));
            ui.add(egui::Label::new(&self.wil_mod_text));


            
            fn change_trait_mod(trait_mod_text: &mut String, new_trait_mod: &mut String, ctx: &egui::Context) {
                trait_mod_text.clear();
                trait_mod_text.push_str(new_trait_mod);
                ctx.request_repaint();
            }
        });}

}
fn main() -> eframe::Result<()> {
    eframe::run_native("Mutagen Character Creature", eframe::NativeOptions::default(),
     Box::new(|ctx| Ok(Box::new(CharacterApp::new(ctx)))))
}
