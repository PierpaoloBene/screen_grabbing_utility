use screenshots::{image::EncodableLayout, Screen};
use std::{sync::mpsc::Receiver, time::Instant};

use eframe::{
    egui::{self, Color32, Options, RichText, Visuals},
    epaint::mutex::Mutex,
    Frame,
};
use egui::Pos2;
use std::fs;
use std::fs::File;

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
        transparent: true,
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Screen Grabbing Utility",
        options,
        Box::new(|_cc| {
            Box::new(FirstWindow {
                selected_mode: ModeOptions::Rectangle,
                selected_mode_string: "Rectangle".to_string(),
                selected_timer: TimerOptions::NoTimer,
                selected_timer_string: "No timer".to_string(),
                selected_window: 1,
                mouse_pos: Option::Some(egui::pos2(-1.0, -1.0)),
                mouse_pos_f: Option::Some(egui::pos2(-1.0, -1.0)),
            })
        }),
    )
}

struct FirstWindow {
    selected_mode: ModeOptions,
    selected_mode_string: String,
    selected_timer: TimerOptions,
    selected_timer_string: String,
    selected_window: usize,
    mouse_pos: Option<Pos2>,
    mouse_pos_f: Option<Pos2>,
}
impl eframe::App for FirstWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        if self.selected_window == 1 {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(20.0); // da modificare
                    if ui
                        .add_sized([50., 50.], egui::Button::new(RichText::new("+").size(30.0)))
                        .clicked()
                    {
                        println!("premuto +");
                        self.selected_window = 2;
                    }

                    egui::ComboBox::from_id_source("mode_Combobox")
                        .width(200.0)
                        .selected_text(
                            RichText::new(format!("{}", self.selected_mode_string)).size(30.0),
                        )
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
                        .selected_text(
                            RichText::new(format!("{}", self.selected_timer_string)).size(30.0),
                        )
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
        } else if self.selected_window == 2 {
            frame.set_decorations(false);
            frame.set_window_size(frame.info().window_info.monitor_size.unwrap());
            frame.set_window_pos(egui::pos2(0.0, 0.0));
           

            egui::Window::new("Second window").show(ctx, |ui| {
                let _start = Instant::now();

                let screens = Screen::all().unwrap();

                if ui.input(|i| {
                    i.pointer.any_down()
                        && self.mouse_pos.unwrap()[0] == -1.0
                        && self.mouse_pos.unwrap()[1] == -1.0
                }) {
                    frame.set_visible(false);
                    println!("salvo pressione");
                    self.mouse_pos = ui.input(|i| i.pointer.interact_pos());
                    // let mut image = Screen::from_point(
                    //     mouse_pos.unwrap()[0] as i32,
                    //     mouse_pos.unwrap()[1] as i32,

                    // );
                }

                if ui.input(|i| i.pointer.any_released()) {
                    println!("salvo rilascio");
                    self.mouse_pos_f = ui.input(|i| i.pointer.interact_pos());
                }

                let width = self.mouse_pos_f.unwrap()[0] - self.mouse_pos.unwrap()[0];
                let height = self.mouse_pos_f.unwrap()[1] - self.mouse_pos.unwrap()[1];

                if self.mouse_pos.unwrap()[0] > -1.0
                    && self.mouse_pos.unwrap()[1] > -1.0
                    && self.mouse_pos_f.unwrap()[0] > -1.0
                    && self.mouse_pos_f.unwrap()[1] > -1.0
                {
                    println!("sono nell'if");
                    for screen in screens {
                        let mut image = screen.capture_area(
                            self.mouse_pos.unwrap()[0] as i32,
                            self.mouse_pos.unwrap()[1] as i32,
                            width as u32,
                            height as u32,
                        );

                        if image.is_err() == false {
                            println!("gira gira gira gira");
                            image.unwrap().save("/Users/pierpaolobene/Documents/ao.jpg");
                            println!("sto resettando");
                            self.selected_window = 1;
                        }
                        //fs::write("C:\\Users\\masci\\Desktop\\ao.jpg", image.unwrap());
                        frame.set_visible(true);
                    }
                }

                //println!("Click del mouse a: {:?}", mouse_pos.unwrap()[0]);
            });
        }
    }
}
