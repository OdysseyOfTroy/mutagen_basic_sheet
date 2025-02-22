mod character;
mod mutation;
mod skills;
mod character_app;
pub mod enums;

mod weapon_proficiencies;

use character_app::CharacterApp;


fn main() -> eframe::Result<()> {
    eframe::run_native("Mutant Automated Assistance Module", eframe::NativeOptions::default(),
     Box::new(|ctx| Ok(Box::new(CharacterApp::new(ctx)))))
}
