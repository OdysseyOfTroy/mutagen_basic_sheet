mod character_app;
pub mod enums;
pub mod character_structs;

use character_app::CharacterApp;


fn main() -> eframe::Result<()> {
    eframe::run_native("Mutant Automated Assistance Module", eframe::NativeOptions::default(),
     Box::new(|ctx| Ok(Box::new(CharacterApp::new(ctx)))))
}
