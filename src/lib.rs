mod state;
use rand::{thread_rng, Rng};
pub use state::State;

pub fn roller(die_size: u32) {
    let mut rng = thread_rng();
    let result: u32 = rng.gen_range(1..=die_size);

    if die_size == 20 {
        match result {
            1 => println!("You got a Nat 1..."),
            20 => println!("You got a Nat 20!"),
            8 => println!("You got an 8."),
            11 => println!("You got an 11."),
            18 => println!("You got an 18."),
            _ => println!("You got a {}.", result),
        }
    } else {
        match result {
            8 => println!("You got an 8."),
            11 => println!("You got an 11."),
            18 => println!("You got an 18."),
            _ => println!("You got a {}.", result),
        }
    }
}
