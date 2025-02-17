mod character;

use character::CharacterApp;
use eframe;


fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Mutagen Character Sheet", options, Box::new(|_cc| Box::new(CharacterApp::default()))).unwrap();
}
