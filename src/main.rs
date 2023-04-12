use derivative::Derivative;
use eframe::run_native;
use egui::{Button, Slider, Color32};
use mini_dice_roller::{enum_to_int, message, roller, Dice, State};

#[derive(Derivative)]
#[derivative(Default)]
struct DiceRoller {
    current_state: State,
    dice_enum: Dice,
    die_size: Option<u32>,
    #[derivative(Default(value = "1"))]
    die_amount: u32,
    die_result: Option<u32>,

    // Text display
    rolling_msg: String,
    individual_results: String,
    result_msg: String,
}

impl eframe::App for DiceRoller {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Welcome to Jelly's Mini Dice Roller");
                egui::widgets::global_dark_light_mode_switch(ui);
            });
            ui.end_row();
            ui.separator();

            let dice_selection = ui.horizontal(|ui| {
                ui.label("🎲 Select a die:");
                if ui
                    .selectable_value(&mut self.dice_enum, Dice::D20, "D20")
                    .clicked()
                {
                    self.current_state = State::Selection;
                };
                if ui
                    .selectable_value(&mut self.dice_enum, Dice::D12, "D12")
                    .clicked()
                {
                    self.current_state = State::Selection;
                };
                if ui
                    .selectable_value(&mut self.dice_enum, Dice::D10, "D10")
                    .clicked()
                {
                    self.current_state = State::Selection;
                };
                if ui
                    .selectable_value(&mut self.dice_enum, Dice::D8, "D8")
                    .clicked()
                {
                    self.current_state = State::Selection;
                };
                if ui
                    .selectable_value(&mut self.dice_enum, Dice::D6, "D6")
                    .clicked()
                {
                    self.current_state = State::Selection;
                };
                if ui
                    .selectable_value(&mut self.dice_enum, Dice::D4, "D4")
                    .clicked()
                {
                    self.current_state = State::Selection;
                };
                if ui
                    .selectable_value(&mut self.dice_enum, Dice::D100, "D100")
                    .clicked()
                {
                    self.current_state = State::Selection;
                };
                enum_to_int(&self.dice_enum)
            });
            ui.end_row();

            if matches!(self.current_state, State::Selection) {
                self.die_size = Some(dice_selection.inner);
            }

            ui.add_space(8.0);

            let is_reset = self
                .die_size
                .map(|die_size| {
                    if ui
                        .add(Slider::new(&mut self.die_amount, 1..=20).text("Amount to roll"))
                        .changed()
                    {
                        self.current_state = State::Selection;
                    };

                    let btn_color_mode = if ui.visuals().dark_mode {
                        Color32::from_rgba_premultiplied(111, 42, 135, 255)
                    } else {
                        Color32::from_rgba_premultiplied(210, 153, 230, 255)
                    };
                    let roll_button = ui.add(
                        Button::new("🎲 Roll!")
                        .fill(btn_color_mode)
                    );

                    if roll_button.clicked() {
                        self.individual_results.clear();
                        self.rolling_msg = format!("Rolling {}d{die_size}...", self.die_amount);

                        let mut total: u32 = 0;
                        for i in 1..=self.die_amount {
                            let indiv_roll = roller(die_size);
                            total += indiv_roll;

                            self.individual_results
                                .push_str(&format!("{}", indiv_roll.to_string()));
                            if self.die_amount == 1 || i == self.die_amount {
                                break;
                            }
                            self.individual_results.push_str(&format!(" + "));
                        }

                        self.die_result = Some(total);
                        self.result_msg = message(
                            self.die_amount,
                            self.die_size.unwrap(),
                            self.die_result.unwrap(),
                        );
                        self.current_state = State::Result;
                    };

                    if matches!(self.current_state, State::Result) {
                        ui.label(&self.rolling_msg);
                        ui.label(&self.result_msg);
                        // Display each die result for multiple rolls
                        if self.die_amount > 1 {
                            ui.label(&self.individual_results);
                        }
                    }
                    ui.button("↻ Reset").clicked()
                })
                .unwrap_or_default();

            if is_reset {
                self.current_state = State::Reset;
                self.die_size = None;
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        min_window_size: Some(egui::Vec2 {
            x: (500.0),
            y: (480.0),
        }),
        initial_window_size: Some(egui::Vec2 {
            x: (500.0),
            y: (480.0),
        }),
        ..Default::default()
    };

    run_native(
        "Mini Dice Roller",
        options,
        Box::new(|_| Box::<DiceRoller>::default()),
    )
}
