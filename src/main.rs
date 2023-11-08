use screenshots::Screen;
use std::time::Duration;

use eframe::{
    egui::{self, Color32, RichText},
    Frame,
};
use egui::{epaint::RectShape, Pos2, Rect, Rounding, Shape, Stroke, Vec2};

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
        transparent: true,
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
                selected_timer_numeric: 0 as u64,
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
    selected_timer_numeric:u64,
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
                                self.selected_timer_numeric=3;
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
                                self.selected_timer_numeric=5;
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
                                self.selected_timer_numeric=10;
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
        }  else if self.selected_window == 2 {
            frame.set_decorations(false);
            frame.set_window_size(frame.info().window_info.monitor_size.unwrap());
            frame.set_window_pos(egui::pos2(0.0, 0.0));


            egui::Area::new("my_area")
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    if ui.input(|i| {
                        i.pointer.any_down()
                            && self.mouse_pos.unwrap()[0] == -1.0
                            && self.mouse_pos.unwrap()[1] == -1.0
                    }) {
                        println!("salvo pressione");

                        self.mouse_pos = ui.input(|i| i.pointer.interact_pos());
                        //self.mouse_pos=self.mouse_pos_2;
                    }
                    if self.mouse_pos.unwrap()[0] != -1.0
                        && self.mouse_pos.unwrap()[1] != -1.0
                    {
                        self.mouse_pos_f = ui.input(|i| i.pointer.latest_pos());
                    }
                    if ui.input(|i| i.pointer.any_released()) {
                        frame.set_window_size(Vec2::new(0.0,0.0));


                        self.selected_window = 3; //Le coordinate sono slavate in self.mouse_pos_2 e self.mouse_posf_2
                    }
                    // if(self.mouse_pos_2.unwrap()[0]<=self.mouse_pos_f_2.unwrap()[0]
                    //   && self.mouse_pos_2.unwrap()[1]<=self.mouse_pos_f_2.unwrap()[1]){
                    ui.painter().add(Shape::Rect(RectShape::new(
                        Rect::from_min_max(self.mouse_pos.unwrap(), self.mouse_pos_f.unwrap()),
                        Rounding::default(),
                        Color32::TRANSPARENT,
                        Stroke::new(2.0,Color32::GRAY),
                    )));
                    //}else if(self.mouse_pos_2.unwrap()[0]=self.mouse_pos_f_2.unwrap()[0]
                    //       && self.mouse_pos_2.unwrap()[1]<=self.mouse_pos_f_2.unwrap()[1]){
                    //  ui.painter().add( Shape::Rect(  RectShape::new(Rect::from_min_max(self.mouse_pos_2.unwrap(), self.mouse_pos_f_2.unwrap()), Rounding::default(), Color32::LIGHT_RED, Stroke::default())));
                    //}
                });
            }else if self.selected_window==3{
                self.selected_window=4;
            }else if self.selected_window == 4 {
            let screens = Screen::all().unwrap();

            let width = self.mouse_pos_f.unwrap()[0] - self.mouse_pos.unwrap()[0];
            let height = self.mouse_pos_f.unwrap()[1] - self.mouse_pos.unwrap()[1];
            std::thread::sleep(Duration::from_secs(self.selected_timer_numeric));
            for screen in screens {
                let image = screen.capture_area(
                    self.mouse_pos.unwrap()[0] as i32,
                    self.mouse_pos.unwrap()[1] as i32,
                    width as u32,
                    height as u32,
                );

                if image.is_err() == false {
                    println!("gira gira gira gira");
                    let _=image
                        .unwrap()
                        .save("/Users/pierpaolobene/Desktop/ao.jpg");
                }
                //fs::write("C:\\Users\\masci\\Desktop\\ao.jpg", image.unwrap());
                println!(
                    "xi={} yi={} xf={} yf={}",
                    self.mouse_pos.unwrap()[0],
                    self.mouse_pos.unwrap()[1],
                    self.mouse_pos.unwrap()[0],
                    self.mouse_pos.unwrap()[1]
                );
            }

            self.selected_window = 5; //Le coordinate sono slavate in self.mouse_pos_2 e self.mouse_posf_2
            //frame.set_window_size(frame.info().window_info.monitor_size.unwrap());
        }else if self.selected_window==5{
            frame.set_decorations(true);
            frame.set_window_size(Vec2::new(1640.0,600.0));
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("caccona");
            });
        }else if self.selected_window==6{
            let screens = Screen::all().unwrap();
            std::thread::sleep(Duration::from_secs(self.selected_timer_numeric));
            for screen in screens {
                let image = screen.capture();

                if image.is_err() == false {
                    println!("gira gira gira gira");
                    
                    let _=image
                        .unwrap()
                        .save("C:\\Users\\masci\\Desktop\\ao.jpg");
                    println!("sto resettando");
                    self.selected_window = 1;
                }
                //fs::write("C:\\Users\\masci\\Desktop\\ao.jpg", image.unwrap());
            }
            self.selected_window=5;
        }
    }
}
