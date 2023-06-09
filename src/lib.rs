mod state;
mod dice;
use rand::{thread_rng, Rng};
pub use state::State;
pub use dice::Dice;

pub fn message(die_amount: u32, die_size: u32, die_result: u32) -> String {
    match (die_amount, die_size, die_result) {
        (1, 20, 1) => String::from("You got a Nat 1..."),
        (_, _, 1) => String::from("You got a 1."),
        (1, 20, 20) => String::from("You got a Nat 20!"),
        (_, _, 20) => String::from("You got a 20."),
        (_, _, 8) => String::from("You got an 8."),
        (_, _, 11) => String::from("You got an 11."),
        (_, _, 18) => String::from("You got an 18."),
        (_, 100, eighties @ 80..=89) => format!("You got an {eighties}."),
        (_, 100, eighthundreds @ 800..=899) => format!("You got an {eighthundreds}."),
        _ => format!("You got a {die_result}."),
    }
}

pub fn roller(die_size: u32) -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(1..=die_size)
}

pub fn enum_to_int(choice: &Dice) -> u32 {
    match choice {
        Dice::D4 => 4,
        Dice::D6 => 6,
        Dice::D8 => 8,
        Dice::D10 => 10,
        Dice::D12 => 12,
        Dice::D20 => 20,
        Dice::D100 => 100,
    }
}