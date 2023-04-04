// use std::io;
use eframe::run_native;
use egui::Button;
use mini_dice_roller::{/*roller,*/ State};

struct DiceRoller {
    current_state: State,
    die_size: u32,
}

impl eframe::App for DiceRoller {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to Jelly's Mini Dice Roller");

            match self.current_state {
                State::Selection => {
                    
                },
                State::Result => {
                    todo!()
                },
            }
        });
    }
}

impl DiceRoller {
    fn new() -> DiceRoller {
        DiceRoller {
            current_state: State::default(),
            die_size: 20,
        }
    }
}


fn main() {
    run_native(
        "Mini Dice Roller",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(DiceRoller::new())),
    ).unwrap();

    println!("Die sizes: [d4] [d6] [d8] [d10] [d12] [d20]");
    // println!("Input the die size:");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();

    // let die_size: u32 = input.trim().parse().unwrap();
    // println!("Rolling a d{}", die_size);

    // roller(die_size);
}
