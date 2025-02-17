mod character;

use character::Character;
fn main() {
    let character = Character::new("Eve", "Nova", 1, 33, 31, 51, 33, 45, 57,);
    character.display();
}
