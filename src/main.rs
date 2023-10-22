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
    let mut selected_timer: TimerOptions = TimerOptions::NoTimer;
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_simple_native("Screen Grabbing Utility", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(100.0); // da modificare
                if ui
                    .add_sized([50., 50.], egui::Button::new(RichText::new("+").size(30.0)))
                    .clicked()
                {
                    println!("caccona");
                }

                egui::ComboBox::from_id_source("mode_Combobox")
                    .width(200.0)
                    .selected_text(RichText::new(format!("{:?}", selected_mode)).size(30.0))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut selected_mode,
                            ModeOptions::Rectangle,
                            RichText::new("Rectangle").size(30.0),
                        );
                        ui.selectable_value(
                            &mut selected_mode,
                            ModeOptions::FullScreen,
                            RichText::new("FullScreen").size(30.0),
                        );
                    });

                egui::ComboBox::from_id_source("timer_Combobox")
                    .width(200.0)
                    .selected_text(RichText::new(format!("{:?}", selected_timer)).size(30.0))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut selected_timer,
                            TimerOptions::NoTimer,
                            RichText::new("No Timer").size(30.0),
                        );
                        ui.selectable_value(
                            &mut selected_timer,
                            TimerOptions::ThreeSeconds,
                            RichText::new("3 Seconds").size(30.0),
                        );
                        ui.selectable_value(
                            &mut selected_timer,
                            TimerOptions::FiveSeconds,
                            RichText::new("5 Seconds").size(30.0),
                        );
                        ui.selectable_value(
                            &mut selected_timer,
                            TimerOptions::TenSeconds,
                            RichText::new("10 Seconds").size(30.0),
                        );
                    });
            });
        });
    })
}
