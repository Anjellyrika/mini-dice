// use std::io;
use eframe::run_native;
use mini_dice_roller::{message, roller, State};

#[derive(Default)]
struct DiceRoller {
    current_state: State,
    die_size: Option<u32>,
    die_result: Option<u32>,

    // Text display
    rolling_msg: String,
    result_msg: String,
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

            let is_reset = self
                .die_size
                .map(|die_size| {
                    ui.label(&self.rolling_msg);
                    ui.label(&self.result_msg);
                    if matches!(self.current_state, State::Selection) {
                        self.rolling_msg = format!("Rolling d{die_size}...");
                        self.die_result = Some(roller(die_size));
                        self.result_msg = message(self.die_size.unwrap(), self.die_result.unwrap());
                        self.current_state = State::Result;
                    }
                    ui.button("Reset").clicked()
                })
                .unwrap_or_default();

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
            } else if is_reset {
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
}
