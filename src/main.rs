// use std::io;
use eframe::run_native;
use mini_dice_roller::{State, roller};

#[derive(Default)]
struct DiceRoller {
    current_state: State,
    die_size: Option<u32>,
    die_result: Option<u32>
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
            let d100 = ui.button("D100");

            let maybe_reset = if let Some(die_size) = self.die_size {
                // FIXME: Potential performance bottleneck due to frequent `malloc` + `free`!
                // => Cache the output in the DiceRoller struct
                ui.label(format!("Rolling d{die_size}..."));
                
                match self.current_state {
                    State::Selection => {
                        roller(die_size);  
                        self.current_state = State::Result;
                    },
                    State::Result => {
                        println!("currently in Result mode");
                    },
                }
                Some(ui.button("Reset"))

            } else {
                None
            };

            self.die_size = if d4.clicked() {
                self.current_state = State::Selection;
                Some(4)
            } else if d6.clicked() {
                self.current_state = State::Selection;
                Some(6)
            } else if d8.clicked() {
                self.current_state = State::Selection;
                Some(8)
            } else if d10.clicked() {
                self.current_state = State::Selection;
                Some(10)
            } else if d12.clicked() {
                self.current_state = State::Selection;
                Some(12)
            } else if d20.clicked() {
                self.current_state = State::Selection;
                Some(20)
            } else if d100.clicked() {
                self.current_state = State::Selection;
                Some(100)
            } else if maybe_reset.as_ref().map(egui::Response::clicked).unwrap_or_default() {
                self.current_state = State::Selection;
                None
            } else {
                return;
            };
        });
    }
}

fn main() -> eframe::Result<()> {
    run_native(
        "Mini Dice Roller",
        eframe::NativeOptions::default(),
        Box::new(|_| Box::<DiceRoller>::default()),
    )

    // println!("Die sizes: [d4] [d6] [d8] [d10] [d12] [d20]");
    // println!("Input the die size:");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();

    // let die_size: u32 = input.trim().parse().unwrap();

    // roller(die_size);
}
