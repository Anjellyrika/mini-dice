mod state;

use rand::{thread_rng, Rng};
pub use state::State;

pub fn message(die_size: u32, die_result: u32) -> String {
    let msg = match die_result {
            1 => {
                if die_size == 20 {
                    String::from("You got a Nat 1...")
                } else {
                    String::from("You got a 1.")
                }
            },
            20 => {
                if die_size == 20 {
                    String::from("You got a Nat 20!")
                } else {
                    String::from("You got a 20.")
                }
            },
            8 => String::from("You got an 8."),
            11 => String::from("You got an 11."),
            18 => String::from("You got an 18."),
            _ => format!("You got a {}.", die_result),
        };
        return msg;
}

pub fn roller(die_size: u32) -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(1..=die_size)
}