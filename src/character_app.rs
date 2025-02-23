use std::path::PathBuf;

use eframe::egui::{self, RichText};
use egui_file_dialog::FileDialog;

use crate::enums::{proficiencies::Proficiency, traits::Traits};
use crate::character_structs::{character::Character, mutation::Mutation};

pub(crate) struct CharacterApp {
    character: Character,
    str_mod_text: String,
    dsc_mod_text: String,
    con_mod_text: String,
    int_mod_text: String,
    sns_mod_text: String,
    wil_mod_text: String,
    melee_strike_text: String,
    range_strike_text: String,
    ability_strike_text: String,
    precision_strike_text: String,
    selected_trait: Traits,
    selected_proficiency: Proficiency,
    selected_trait_value: u8,
    selected_trait_value_text: String,
    calculated_cam_value: u8,
    displayed_cam_value: u8,
    displayed_cam_value_text: String,
    calculated_cam_crit_success_value: u8,
    calculated_cam_crit_success_value_text: String,
    calculated_cam_crit_fail_value: u8,
    calculated_cam_crit_fail_value_text: String,
    misc_mod_value: i8,
    mutations: Vec<Mutation>,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>, 
    save_dialog: FileDialog
}


impl CharacterApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let character = Character::default();
        let mutations = Mutation::default();
        let selected_trait = Traits::Strength;
        let selected_proficiency = Proficiency::Untrained;
        let selected_trait_value = character.strength;
        let selected_trait_value_text = character.strength.to_string();

        let calculated_cam_value = selected_trait_value;
        let displayed_cam_value = calculated_cam_value;
        let displayed_cam_value_text = format!("{} %", displayed_cam_value);
        let calculated_cam_crit_success_value = Character::calculate_crit_success(calculated_cam_value);
        let calculated_cam_crit_success_value_text = calculated_cam_crit_success_value.to_string();
        let calculated_cam_crit_fail_value = Character::calculate_crit_fail(calculated_cam_value);
        let calculated_cam_crit_fail_value_text = calculated_cam_crit_fail_value.to_string();

        let misc_mod_value = 0;

        let range_strike = character.sense;
        let melee_strike = character.strength;
        let ability_strike = Character::calculate_ability_strike_trait(&character);
        let precision_strike = character.discipline;

        let range_strike_text = range_strike.to_string();
        let melee_strike_text = melee_strike.to_string();
        let ability_strike_text = ability_strike.to_string();
        let precision_strike_text = precision_strike.to_string();

        let str_mod_text = Character::calculate_mod(character.strength);
        let dsc_mod_text = Character::calculate_mod(character.discipline);
        let con_mod_text = Character::calculate_mod(character.constitution);
        let int_mod_text = Character::calculate_mod(character.intelligence);
        let sns_mod_text = Character::calculate_mod(character.sense);
        let wil_mod_text = Character::calculate_mod(character.will);

        Self { character, mutations, file_dialog: FileDialog::new(), picked_file: None, save_dialog: FileDialog::new(),
            str_mod_text, dsc_mod_text, con_mod_text, int_mod_text, sns_mod_text, wil_mod_text, 
            range_strike_text, melee_strike_text, ability_strike_text, precision_strike_text, 
            selected_trait,selected_trait_value, selected_trait_value_text, 
            selected_proficiency, 
            calculated_cam_value,displayed_cam_value, displayed_cam_value_text,
            calculated_cam_crit_success_value, calculated_cam_crit_success_value_text,
            calculated_cam_crit_fail_value, calculated_cam_crit_fail_value_text,
            misc_mod_value,
        }
    }

    fn c_a_m(&mut self, trait_value: u8, proficiency_value: u8, misc_mod: i8, ctx: &egui::Context) {
        self.calculated_cam_value = Character::calculate_success(trait_value, proficiency_value, misc_mod);
        self.displayed_cam_value = calculate_display_success(self.calculated_cam_value);
        change_label(&mut self.displayed_cam_value_text, &mut to_percentage_string(self.displayed_cam_value), ctx);
        
        self.calculated_cam_crit_success_value = Character::calculate_crit_success(self.calculated_cam_value);
        change_label(&mut self.calculated_cam_crit_success_value_text, &mut self.calculated_cam_crit_success_value.to_string(), ctx);

        self.calculated_cam_crit_fail_value = Character::calculate_crit_fail(self.calculated_cam_value);
        change_label(&mut self.calculated_cam_crit_fail_value_text, &mut self.calculated_cam_crit_fail_value.to_string(), ctx);
    }

    fn matching_trait_checks(&mut self, this_trait: Traits, ctx: &egui::Context) {
        //change ability strike label
        if determine_matching_trait(&mut self.character.mutation.main_trait, &this_trait) {
            let mut new_ability_strike = Character::calculate_ability_strike_trait(&self.character).to_string();
            change_label(&mut self.ability_strike_text, &mut new_ability_strike, ctx);
        }
        //change selected trait value label
        if determine_matching_trait(&mut self.selected_trait, &this_trait) {
            let new_selected_trait_value = Character::get_trait_value(&self.character, &self.selected_trait);
            self.selected_trait_value = new_selected_trait_value;
            change_label(&mut self.selected_trait_value_text, &mut new_selected_trait_value.to_string(), ctx);

            self.c_a_m(self.selected_trait_value, self.selected_proficiency.value(),self.misc_mod_value, ctx);
        }
    }
}

fn determine_matching_trait(main_trait: &mut Traits, this_trait: &Traits) -> bool {
    if main_trait == this_trait {
        true
    } else {
        false
    }
}

pub fn calculate_display_success(cam_value: u8) -> u8 {
    if cam_value >= 100 {
        99
    } else {
        cam_value
    }
}

pub fn to_percentage_string(value: u8) -> String {
    format!("{} %", value)
}

pub fn change_label(label_text: &mut String, new_label_text: &mut String, ctx: &egui::Context) {
    label_text.clear();
    label_text.push_str(new_label_text);
    ctx.request_repaint();
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

                //update character values from picked file
                self.picked_file = Some(path.to_path_buf());
                let character_path_str = path.to_string_lossy().to_string();
                self.character = Character::from_json(&character_path_str);

                //update mod values
                self.str_mod_text = Character::calculate_mod(self.character.strength);
                self.dsc_mod_text = Character::calculate_mod(self.character.discipline);
                self.con_mod_text = Character::calculate_mod(self.character.constitution);
                self.int_mod_text = Character::calculate_mod(self.character.intelligence);
                self.sns_mod_text = Character::calculate_mod(self.character.sense);
                self.wil_mod_text = Character::calculate_mod(self.character.will);
        
                //update strike values
                self.range_strike_text = self.character.sense.to_string();
                self.melee_strike_text = self.character.strength.to_string();
                self.ability_strike_text = Character::calculate_ability_strike_trait(&self.character).to_string();
                self.precision_strike_text = self.character.discipline.to_string();

                //update selected trait value
                self.selected_trait_value = Character::get_trait_value(&self.character, &self.selected_trait);
                self.selected_trait_value_text = self.selected_trait_value.to_string();

                //update calculated cam values
                self.c_a_m(self.selected_trait_value, self.selected_proficiency.value(),self.misc_mod_value, ctx);
            }

            

            if let Some(path) = self.save_dialog.update(ctx).picked() {
                let saving_path_str = path.to_string_lossy().to_string();
                if let Err(err) = self.character.to_json(&saving_path_str) {
                    eprintln!("Failed to save character: {}", err);
                }};
        });
        egui::SidePanel::right("CAM").show(ctx, |ui| {
            ui.heading("CAM");
            ui.label("Trait");
            egui::ComboBox::from_id_salt("Trait")
                .selected_text(&self.selected_trait.to_string())
                .show_ui(ui, |ui|{
                    for main_trait in Traits::iterator() {
                        if ui.selectable_value(&mut self.selected_trait, main_trait.clone(), &main_trait.to_string()).clicked() {
                            //change selected trait value
                            let new_selected_trait_value = Character::get_trait_value(&self.character, &self.selected_trait);
                            self.selected_trait_value = new_selected_trait_value;
                            change_label(&mut self.selected_trait_value_text, &mut new_selected_trait_value.to_string(), ctx);

                            self.c_a_m(new_selected_trait_value, self.selected_proficiency.value(),self.misc_mod_value, ctx);
                        }
                    }
                });
            ui.label(&self.selected_trait_value_text);
            for prof_level in Proficiency::iterator() {
                if ui.add(egui::RadioButton::new(self.selected_proficiency == prof_level.clone(), &prof_level.to_string())).clicked()
                {
                    self.selected_proficiency = prof_level;
            
                    self.c_a_m(self.selected_trait_value, self.selected_proficiency.value(),self.misc_mod_value, ctx);
                // change_label(&mut self.calculated_cam_crit_fail_value_text, &mut self.calculated_cam_crit_fail_value.to_string(), ctx);
                };
                    
            };

            if ui.add(egui::Slider::new(&mut self.misc_mod_value, -50..=50).step_by(5.0)).changed() {
                self.c_a_m(self.selected_trait_value, self.selected_proficiency.value(),self.misc_mod_value, ctx);
            }


            ui.label("Success:");
            ui.label(RichText::new(&self.displayed_cam_value_text).size(30.0));
    
            ui.label("Crit Success:");
            ui.label(RichText::new(&self.calculated_cam_crit_success_value_text).size(30.0));
    
            ui.label("Crit Fail:");
            ui.label(RichText::new(&self.calculated_cam_crit_fail_value_text).size(30.0))

        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mutant Assessment");
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("traits_grid").show(ui, |ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.character.name);
                    ui.end_row();
                    egui::ComboBox::from_label("Mutation")
                        .selected_text(&self.character.mutation.name)
                        .show_ui(ui, |ui|{
                            for mutation in &self.mutations {
                                if ui.selectable_value(&mut self.character.mutation, mutation.clone(), &mutation.name).clicked() {
                                    let mut new_ability_strike = Character::calculate_ability_strike_trait(&self.character).to_string();
                                    change_label(&mut self.ability_strike_text, &mut new_ability_strike, ctx);
                                }
                            }
                        }); 
                        ui.end_row();
                    ui.label("main trait:");
                    ui.label(&self.character.mutation.main_trait.to_string());
                    ui.end_row();
                    ui.label("Threat Level:");
                    ui.add(egui::Slider::new(&mut self.character.threat, 1..=10));
                    ui.end_row();
                    //Trait Section
                    ui.label("Strength:");
                    if ui.add(egui::Slider::new(&mut self.character.strength, 1..=100)).changed() {
                        let mut new_str_mod = ((self.character.strength as i8 / 10 as i8) - 2 as i8).to_string();
                        change_label(&mut self.str_mod_text, &mut new_str_mod, ctx);
                        self.matching_trait_checks(Traits::Strength, ctx);
                        change_label(&mut self.melee_strike_text, &mut self.character.strength.to_string(), ctx);
                    };
                    ui.add(egui::Label::new(&self.str_mod_text));
                    ui.end_row();

                    ui.label("Discipline:");
                    if ui.add(egui::Slider::new(&mut self.character.discipline, 1..=100)).changed() {
                        let mut new_dsc_mod = ((self.character.discipline as i8 / 10 as i8) - 2 as i8).to_string();
                        change_label(&mut self.dsc_mod_text, &mut new_dsc_mod, ctx);
                        self.matching_trait_checks(Traits::Discipline, ctx);
                        change_label(&mut self.precision_strike_text, &mut self.character.discipline.to_string(), ctx);
                    };
                    ui.add(egui::Label::new(&self.dsc_mod_text));
                    ui.end_row();
                    ui.label("Constitution:");
                    if ui.add(egui::Slider::new(&mut self.character.constitution, 1..=100)).changed() {
                        let mut new_con_mod = ((self.character.constitution as i8 / 10 as i8) - 2 as i8).to_string();
                        change_label(&mut self.con_mod_text, &mut new_con_mod, ctx);
                        self.matching_trait_checks(Traits::Constitution, ctx);
                    };
                    ui.add(egui::Label::new(&self.con_mod_text));
                    ui.end_row();

                    ui.label("Intelligence:");
                    if ui.add(egui::Slider::new(&mut self.character.intelligence, 1..=100)).changed() {
                        let mut new_int_mod = ((self.character.intelligence as i8 / 10 as i8) - 2 as i8).to_string();
                        change_label(&mut self.int_mod_text, &mut new_int_mod, ctx);
                        self.matching_trait_checks(Traits::Intelligence, ctx);
                    };
                    ui.add(egui::Label::new(&self.int_mod_text));
                    ui.end_row();

                    ui.label("Sense:");
                    if ui.add(egui::Slider::new(&mut self.character.sense, 1..=100)).changed() {
                        let mut new_sns_mod = ((self.character.sense as i8 / 10 as i8) - 2 as i8).to_string();
                        change_label(&mut self.sns_mod_text, &mut new_sns_mod, ctx);
                        self.matching_trait_checks(Traits::Sense, ctx);
                        change_label(&mut self.range_strike_text, &mut self.character.sense.to_string(), ctx);
                    };
                    ui.add(egui::Label::new(&self.sns_mod_text));
                    ui.end_row();
                    
                    ui.label("Will:");
                    if ui.add(egui::Slider::new(&mut self.character.will, 1..=100)).changed() {
                        let mut new_wil_mod = ((self.character.will as i8 / 10 as i8) - 2 as i8).to_string();
                        change_label(&mut self.wil_mod_text, &mut new_wil_mod, ctx);
                        self.matching_trait_checks(Traits::Will, ctx);
                    };
                    ui.add(egui::Label::new(&self.wil_mod_text));
                    ui.end_row();
                    
                    ui.separator();
                    ui.end_row();

                    //Strike Section
                    ui.label("Strikes");
                    ui.end_row();
                    ui.label("Melee Strike");
                    ui.label(&self.melee_strike_text);
                    ui.end_row();
                    ui.label("Ranged Strike");
                    ui.label(&self.range_strike_text);
                    ui.end_row();
                    ui.label("Precision Strike");
                    ui.label(&self.precision_strike_text);
                    ui.end_row();
                    ui.label("Ability Strike");
                    ui.label(&self.ability_strike_text);
                    ui.end_row();

                    //Skills Section
                    ui.label("Skills");
                    ui.end_row();
                    ui.separator();
                    ui.end_row();
        
                    for skill in &mut self.character.skills {
                        ui.label(&skill.name);
                        for prof_level in Proficiency::iterator() {
                            ui.radio_value(&mut skill.proficiency_level, prof_level, &prof_level.to_string());
                        }
                        ui.end_row();
                    }
                    
                    ui.separator();
                    ui.end_row();

                    //Weapon Proficiency Section
                    ui.label("Weapons");
                    ui.end_row();
                    ui.separator();
                    ui.end_row();
        
                    for wep_prof in &mut self.character.weapon_proficiencies {
                        ui.label(&wep_prof.name);
                        for prof_level in Proficiency::iterator() {
                            ui.radio_value(&mut wep_prof.proficiency_level, prof_level, &prof_level.to_string());
                        }
                        ui.end_row();
                    }
                    
                })
            });
        });

    }}