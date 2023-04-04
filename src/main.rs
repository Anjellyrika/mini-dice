use std::io;
use rand::{Rng, thread_rng};

fn main() {
    println!("Die sizes: [d4] [d6] [d8] [d10] [d12] [d20]");
    println!("Input the die size:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let die_size: u32 = input.trim().parse().unwrap();
    println!("Rolling a d{}", die_size);

    roller(die_size);
}
