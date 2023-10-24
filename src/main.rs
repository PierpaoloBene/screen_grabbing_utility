use eframe::{
    egui::{self, RichText},
    Frame,
};

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

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Screen Grabbing Utility", 
    options,
     Box::new(|_cc|
        Box::new(
            MyApp{selected_mode: ModeOptions::Rectangle,
                selected_mode_string: "Rectangle".to_string(),
                selected_timer: TimerOptions::NoTimer,
                selected_timer_string: "No timer".to_string()}
        )
    ),
    )
}

struct MyApp {
    selected_mode: ModeOptions,
    selected_mode_string: String,
    selected_timer: TimerOptions,
    selected_timer_string: String,

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(20.0); // da modificare
                if ui
                    .add_sized([50., 50.], egui::Button::new(RichText::new("+").size(30.0)))
                    .clicked()
                {
                    println!("premuto +");
                    frame.set_minimized(true);
                }

                egui::ComboBox::from_id_source("mode_Combobox")
                    .width(200.0)
                    .selected_text(RichText::new(format!("{}", self.selected_mode_string)).size(30.0))
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut self.selected_mode,
                                ModeOptions::Rectangle,
                                RichText::new("Rectangle").size(30.0),
                            )
                            .clicked()
                        {
                            self.selected_mode_string = "Rectangle".to_string();
                        }
                        if ui
                            .selectable_value(
                                &mut self.selected_mode,
                                ModeOptions::FullScreen,
                                RichText::new("Full Screen").size(30.0),
                            )
                            .clicked()
                        {
                            self.selected_mode_string = "Full Screen".to_string();
                        };
                    });

                egui::ComboBox::from_id_source("timer_Combobox")
                    .width(200.0)
                    .selected_text(RichText::new(format!("{}", self.selected_timer_string)).size(30.0))
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut self.selected_timer,
                                TimerOptions::NoTimer,
                                RichText::new("No Timer").size(30.0),
                            )
                            .clicked()
                        {
                            self.selected_timer_string = "No Timer".to_string();
                        };

                        if ui
                            .selectable_value(
                                &mut self.selected_timer,
                                TimerOptions::ThreeSeconds,
                                RichText::new("3 Seconds").size(30.0),
                            )
                            .clicked()
                        {
                            self.selected_timer_string = "3 Seconds".to_string();
                        };
                        if ui
                            .selectable_value(
                                &mut self.selected_timer,
                                TimerOptions::FiveSeconds,
                                RichText::new("5 Seconds").size(30.0),
                            )
                            .clicked()
                        {
                            self.selected_timer_string = "5 Seconds".to_string();
                        };
                        if ui
                            .selectable_value(
                                &mut self.selected_timer,
                                TimerOptions::TenSeconds,
                                RichText::new("10 Seconds").size(30.0),
                            )
                            .clicked()
                        {
                            self.selected_timer_string = "10 Seconds".to_string();
                        };
                    });
                if ui
                    .add_sized(
                        [50., 50.],
                        egui::Button::new(RichText::new("Settings").size(30.0)),
                    )
                    .clicked()
                {
                    println!("premuto Settings");
                }
            });
        });
    }
}
