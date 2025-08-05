use core::time;
use std::thread::sleep;

use rand::Rng;

fn main() {
    loop {
        let dice_roll = roll_dice();
        match dice_roll {
            3 => println!("you got a cool ass hat"),
            7 => println!("the cool ass hat was removed from your inventory (if you had one)"),
            _ => (), // nothing happens if you don't roll a 3 or a 7
        }
        sleep(time::Duration::from_millis(500));
    }
}

fn move_player(distance: u8) {
    println!("Player moved by {distance} meters.")
}

fn roll_dice() -> u8 {
    let mut rng = rand::rng();
    rng.random_range(1..=7)
}
