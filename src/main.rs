use std::io;

fn main() {
    println!("Die sizes: [d4] [d6] [d8] [d10] [d12] [d20]");
    println!("Input the die size:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    print!("Rolling a d{}", input);
}
