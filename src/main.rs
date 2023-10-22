use eframe::egui::{self, RichText};

#[derive(PartialEq, Debug)]
enum ModeOptions {
    Rectangle,
    FullScreen,
}

#[derive(PartialEq, Debug)]
enum TimerOptions {
    NoTimer,
    ThreeSeconds,
    FiveSeconds,
    TenSeconds,
}

fn main() -> Result<(), eframe::Error> {
    let mut selected_mode: ModeOptions = ModeOptions::Rectangle;
    let mut selected_mode_string = "Select Mode".to_string();
    let mut selected_timer: TimerOptions = TimerOptions::NoTimer;
    let mut selected_timer_string = "Set Timer".to_string();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_simple_native("Screen Grabbing Utility", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(20.0); // da modificare
                if ui
                    .add_sized([50., 50.], egui::Button::new(RichText::new("+").size(30.0)))
                    .clicked()
                {
                    println!("premuto +");
                }

                egui::ComboBox::from_id_source("mode_Combobox")
                    .width(200.0)
                    .selected_text(RichText::new(format!("{}", selected_mode_string)).size(30.0))
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut selected_mode,
                                ModeOptions::Rectangle,
                                RichText::new("Rectangle").size(30.0),
                            )
                            .clicked()
                        {
                            selected_mode_string = "Rectangle".to_string();
                        }
                        if ui
                            .selectable_value(
                                &mut selected_mode,
                                ModeOptions::FullScreen,
                                RichText::new("Full Screen").size(30.0),
                            )
                            .clicked()
                        {
                            selected_mode_string = "Full Screen".to_string();
                        };
                    });

                egui::ComboBox::from_id_source("timer_Combobox")
                    .width(200.0)
                    .selected_text(RichText::new(format!("{}", selected_timer_string)).size(30.0))
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut selected_timer,
                                TimerOptions::NoTimer,
                                RichText::new("No Timer").size(30.0),
                            )
                            .clicked()
                        {
                            selected_timer_string = "No Timer".to_string();
                        };
                        if ui
                            .selectable_value(
                                &mut selected_timer,
                                TimerOptions::ThreeSeconds,
                                RichText::new("3 Seconds").size(30.0),
                            )
                            .clicked()
                        {
                            selected_timer_string = "3 Seconds".to_string();
                        };
                        if ui
                            .selectable_value(
                                &mut selected_timer,
                                TimerOptions::FiveSeconds,
                                RichText::new("5 Seconds").size(30.0),
                            )
                            .clicked()
                        {
                            selected_timer_string = "5 Seconds".to_string();
                        };
                        if ui
                            .selectable_value(
                                &mut selected_timer,
                                TimerOptions::TenSeconds,
                                RichText::new("10 Seconds").size(30.0),
                            )
                            .clicked()
                        {
                            selected_timer_string = "10 Seconds".to_string();
                        };
                    });
                if ui
                    .add_sized([50., 50.], egui::Button::new(RichText::new("Settings").size(30.0)))
                    .clicked()
                {
                    println!("premuto Settings");
                }
            });
        });
    })
}
