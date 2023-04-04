use std::io;
use eframe::run_native;
use mini_dice_roller::{roller, State};

struct DiceRoller {
    current_state: State,
}

impl DiceRoller {
    fn new() -> DiceRoller {
        DiceRoller {
            current_state: State::default()
        }
    }
}

fn main() {
    run_native(
        "Mini Dice Roller",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new()),
    )

    println!("Die sizes: [d4] [d6] [d8] [d10] [d12] [d20]");
    println!("Input the die size:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let die_size: u32 = input.trim().parse().unwrap();
    println!("Rolling a d{}", die_size);

    roller(die_size);
}
