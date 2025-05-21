mod battle; // declare the battle module

use crate::battle::Unit; // import Unit and Ability from the unit module

fn main() {
    let mut zoro = Unit::new(String::from("Zoro"), 100, 20, 15);
    let mut kaku = Unit::new(String::from("Kaku"), 80, 15, 10);

    // Simulate turn-based combat
    for i in 0..5 {
        println!("Turn {}: ", i + 1);
        zoro.take_turn(&mut kaku);
        if !kaku.is_alive() {
            println!("{} has been defeated!", kaku.get_name());
            break;
        }
        kaku.take_turn(&mut zoro);
        if !zoro.is_alive() {
            println!("{} has been defeated!", zoro.get_name());
            break;
        }
    }
}
