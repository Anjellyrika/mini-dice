mod state;

use rand::{thread_rng, Rng};
pub use state::State;

pub fn message(die_size: u32, die_result: u32) -> String {
    match (die_size, die_result) {
        (20, 1) => String::from("You got a Nat 1..."),
        (_, 1) => String::from("You got a 1."),
        (20, 20) => String::from("You got a Nat 20!"),
        (_, 20) => String::from("You got a 20."),
        (_, 8) => String::from("You got an 8."),
        (_, 11) => String::from("You got an 11."),
        (_, 18) => String::from("You got an 18."),
        (100, eighties @ 80..=89) => format!("You got an {eighties}."),
        _ => format!("You got a {die_result}."),
    }
}

pub fn roller(die_size: u32) -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(1..=die_size)
}
