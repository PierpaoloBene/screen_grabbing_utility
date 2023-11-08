use screenshots::{image::EncodableLayout, Screen};
use std::{
    default,
    fs::File,
    sync::mpsc::{Receiver, Sender, SyncSender},
    time::{Duration, Instant},
};

use eframe::{
    egui::{self, Color32, Options, RichText, Visuals},
    epaint::mutex::Mutex,
    Frame,
};
use egui::{Pos2, RawInput, Style, Ui, Widget};

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
    let (tx,rx) = std::sync::mpsc::sync_channel(0);
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
                tx: tx,
                rx: rx,
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
    tx: SyncSender<bool>,
    rx: Receiver<bool>
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

                        //agggiunere richiesto screen 
                        if self.selected_mode == ModeOptions::FullScreen {
                            self.tx.send(true);
                            //frame.set_minimized(true);
                        }

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
            
            let mut timer = 0;
            match self.selected_timer {
                TimerOptions::NoTimer => {
                    timer = 0;
                }
                TimerOptions::ThreeSeconds => {
                    timer = 3;
                }
                TimerOptions::FiveSeconds => {
                    timer = 5;
                }
                TimerOptions::TenSeconds => {
                    timer = 10;
                }
                _ => {}
            }
            match self.selected_mode {
                ModeOptions::FullScreen => {
                    if(self.rx.recv().unwrap() == true){
                        
                    frame.set_minimized(true);
                    std::thread::sleep(Duration::from_secs(timer as u64));
                    let screens = Screen::all().unwrap();

                    for screen in screens {
                        println!("screen done");
                        //SCREEN SU TUTTI GLI SCHERMI COLLEGATI O SOLO SU QUELLO IN CUI è APERTA L'APP?

                        let mut image = screen.capture().unwrap();
                        //AGGIUNGERE QUALCOSA PER FAR CAPIRE CHE è STATO FATTO LO SCREEN
                        image
                            //CAMBIARE PATH SALVATAGGIO
                            .save(format!("target/{}.png", screen.display_info.id))
                            .unwrap();
                    }

                    self.selected_window = 1; //CAMBIARE CAMBIO FINESTRA

                    }

                }
                ModeOptions::Rectangle => {
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
                            //frame.set_visible(false);
                            println!("salvo pressione");
                            self.mouse_pos = ui.input(|i| i.pointer.interact_pos());
                        }

                        if ui.input(|i| {
                            i.pointer.any_released()
                                && self.mouse_pos_f.unwrap()[0] == -1.0
                                && self.mouse_pos_f.unwrap()[1] == -1.0
                        }) {
                            println!("salvo rilascio");
                            self.mouse_pos_f = ui.input(|i| i.pointer.interact_pos());
                            frame.set_visible(false);
                            
                        }

                        let width = self.mouse_pos_f.unwrap()[0] - self.mouse_pos.unwrap()[0];
                        let height = self.mouse_pos_f.unwrap()[1] - self.mouse_pos.unwrap()[1];

                        if self.mouse_pos.unwrap()[0] != -1.0
                            && self.mouse_pos.unwrap()[1] != -1.0
                            && self.mouse_pos_f.unwrap()[0] != -1.0
                            && self.mouse_pos_f.unwrap()[1] != -1.0
                        {
                            println!("sono nell'if");
                            //std::thread::sleep(Duration::from_secs(30));

                            for screen in screens {
                                println!("pronto a screennare {:?}", Instant::now());
                                let image = screen.capture_area(
                                    self.mouse_pos.unwrap()[0] as i32,
                                    self.mouse_pos.unwrap()[1] as i32,
                                    width as u32,
                                    height as u32,
                                );

                                if image.is_err() == false {
                                    println!("gira gira gira gira");
                                    let _=image.unwrap().save(format!(
                                        "C:\\Users\\masci\\Desktop\\ao{}.jpg",
                                        screen.display_info.id
                                    ));
                                    println!("sto resettando");
                                    self.selected_window = 1;
                                }
                                //fs::write("C:\\Users\\masci\\Desktop\\ao.jpg", image.unwrap());
                                //frame.set_visible(true);
                            }
                        }

                        //println!("Click del mouse a: {:?}", mouse_pos.unwrap()[0]);
                    });
                }
                _ => {}
            }
        }
    }
}
