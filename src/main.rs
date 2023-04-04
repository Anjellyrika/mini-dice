// use std::io;
use eframe::run_native;
use mini_dice_roller::State;

struct DiceRoller {
    current_state: State,
    die_size: u32,
}

impl eframe::App for DiceRoller {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to Jelly's Mini Dice Roller");

            let d4 = ui.button("D4");
            let d6 = ui.button("D6");
            let d8 = ui.button("D8");
            let d10 = ui.button("D10");
            let d12 = ui.button("D12");
            let d20 = ui.button("D20");

            match self.current_state {
                State::Selection => {
                    self.die_size = if d4.clicked() {
                        4
                    } else if d6.clicked() {
                        6
                    } else if d8.clicked() {
                        8
                    } else if d10.clicked() {
                        10
                    } else if d12.clicked() {
                        12
                    } else if d20.clicked() {
                        20
                    } else {
                        self.die_size
                    };

                    ui.label(self.die_size.to_string());
                }
                State::Result => {
                    todo!()
                }
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
    )
    .unwrap();

    println!("Die sizes: [d4] [d6] [d8] [d10] [d12] [d20]");
    // println!("Input the die size:");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();

    // let die_size: u32 = input.trim().parse().unwrap();
    // println!("Rolling a d{}", die_size);

    // roller(die_size);
}
