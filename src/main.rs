mod post_processing;
use crate::post_processing::PpOptions;
use crate::post_processing::View;
use chrono;
use egui::CursorIcon;
use egui::ImageData;
use egui::Response;
use rfd::FileDialog;

mod functions;
use functions::first_window;

use display_info::DisplayInfo;
use eframe::{
    egui::{self, Color32, RichText},
    Frame,
};
use egui::{epaint::RectShape, Pos2, Rect, Rounding, Shape, Stroke, TextureHandle, Vec2};

use screenshots::Screen;
use std::path::PathBuf;
use std::time::Duration;

use global_hotkey::{
    hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyEventReceiver, GlobalHotKeyManager, HotKeyState,
};
use keyboard_types::{Code, Modifiers};

#[derive(PartialEq, Debug)]
enum ModeOptions {
    Rectangle,
    FullScreen,
}
#[derive(PartialEq, Debug)]
enum Shapes {
    None,
    Arrow,
    Circle,
    Square,
}

#[derive(PartialEq, Debug)]
enum TimerOptions {
    NoTimer,
    ThreeSeconds,
    FiveSeconds,
    TenSeconds,
}

#[derive(PartialEq, Debug)]
enum LoadingState {
    Loaded,
    NotLoaded,
}

#[derive(PartialEq, Debug)]
enum ImageFormat {
    Jpg,
    Png,
    Gif,
}

fn main() -> Result<(), eframe::Error> {
    let mut filepath = Some(PathBuf::new());

    let current_os = if cfg!(unix) {
        let _ = std::fs::create_dir("./screenshot");
        filepath = Some(PathBuf::from("./screenshot"));
        "unix"
    } else if cfg!(windows) {
        let _ = std::fs::create_dir(".//screenshot");
        filepath = Some(PathBuf::from(".//screenshot"));
        "windows"
    } else {
        "unknown"
    };

    println!("{:?}", filepath);
    println!("{:?}", current_os);

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(680.0, 480.0)),
        transparent: true,
        ..Default::default()
    };

    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey_exit = HotKey::new(None, Code::Escape);
    let hotkey_screen = HotKey::new(Some(Modifiers::CONTROL), Code::KeyD);
    let p = post_processing::Painting::default();

    manager.register(hotkey_exit).unwrap();
    manager.register(hotkey_screen).unwrap();

    let openfw = GlobalHotKeyEvent::receiver();
    eframe::run_native(
        "Screen Grabbing Utility",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(FirstWindow {
                screen_to_show: None,
                image_name: None,
                image_format: Some(ImageFormat::Jpg),
                image_format_string: "jpg".to_string(),
                pp_option: None,
                current_os: current_os.to_string(),
                multiplication_factor: None,
                loading_state: LoadingState::NotLoaded,
                image: None,
                image_texture: None,
                filepath: filepath,
                selected_mode: ModeOptions::Rectangle,
                selected_mode_string: "Rectangle".to_string(),
                selected_timer: TimerOptions::NoTimer,
                selected_timer_string: "No timer".to_string(),
                selected_timer_numeric: 0 as u64,
                selected_shape: Shapes::None,
                selected_shape_string: "Select a shape!".to_string(),
                selected_window: 1,
                mouse_pos: Option::Some(egui::pos2(-1.0, -1.0)),
                mouse_pos_f: Option::Some(egui::pos2(-1.0, -1.0)),
                rect_pos: egui::pos2(0.0, 0.0),
                rect_pos_f: egui::pos2(0.0, 0.0),
                open_fw: openfw.clone(),
                screenshots_taken: Vec::new(),
                painting: p,
                width: 0.0,
                height: 0.0,
            })
        }),
    )
}

struct FirstWindow {
    screen_to_show: Option<u32>,
    image_name: Option<String>,
    image_format: Option<ImageFormat>,
    image_format_string: String,
    pp_option: Option<PpOptions>,
    current_os: String,
    multiplication_factor: Option<f32>,
    loading_state: LoadingState,
    image: Option<TextureHandle>,
    image_texture: Option<egui::ColorImage>,
    filepath: Option<PathBuf>,
    selected_mode: ModeOptions,
    selected_mode_string: String,
    selected_timer: TimerOptions,
    selected_timer_string: String,
    selected_timer_numeric: u64,
    selected_shape: Shapes,
    selected_shape_string: String,
    selected_window: usize,
    mouse_pos: Option<Pos2>,
    mouse_pos_f: Option<Pos2>,
    rect_pos: Pos2,
    rect_pos_f: Pos2,
    open_fw: GlobalHotKeyEventReceiver,
    screenshots_taken: Vec<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    painting: post_processing::Painting,
    width: f32,
    height: f32,
}

impl eframe::App for FirstWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        if self.multiplication_factor.is_none() {
            self.multiplication_factor = frame.info().native_pixels_per_point;
        }
        match self.open_fw.try_recv() {
            Ok(event) => match event.state {
                HotKeyState::Pressed => match event.id {
                    2439345500 => {
                        self.selected_window = 1;
                        frame.set_decorations(true);
                        frame.set_window_size(egui::vec2(640.0, 480.0));
                        println!("premuto ESC");
                    }
                    2440410256 => {
                        self.selected_window = 2;

                        println!("premuto ctrl+D");
                    }
                    _ => {
                        println!("siiium")
                    }
                },
                HotKeyState::Released => {}
            },

            Err(_) => {}
            _ => {
                println!("waiting")
            }
        }

        if self.selected_window == 1 {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(20.0); // da modificare
                    if ui
                        .add_sized([50., 50.], egui::Button::new(RichText::new("+").size(30.0))).on_hover_text("Ctrl+D")
                        .clicked()
                    {
                        println!("premuto +");
                        std::thread::sleep(Duration::from_secs(self.selected_timer_numeric));
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
                                self.selected_timer_numeric = 3;
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
                                self.selected_timer_numeric = 5;
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
                                self.selected_timer_numeric = 10;
                            };
                        });
                    if ui
                        .add_sized(
                            [50., 50.],
                            egui::Button::new(RichText::new("⚙ Settings").size(30.0)),
                        )
                        .clicked()
                    {
                        self.selected_window = 6;
                    }
                });
                ui.add_space(150.0);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.label(RichText::new("CTRL+D to take a screenshot").size(30.0));
                });
            });
        } else if self.selected_window == 2 {
            frame.set_decorations(false);
            frame.set_window_size(frame.info().window_info.monitor_size.unwrap() * 2.0);
            //frame.set_window_size(frame.info().window_info.monitor_size.unwrap());
            frame.set_window_pos(egui::pos2(0.0, 0.0));
            
            match self.selected_mode {
                ModeOptions::Rectangle => {
                    egui::Area::new("my_area")
                        .fixed_pos(egui::pos2(0.0, 0.0))
                        .show(ctx, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                ui.label(RichText::new("ESC to go back").size(25.0));
                            });
                            ui.ctx()
                            .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
                            if ui.input(|i| i.pointer.is_moving()) {
                                // println!("{:?}", self.mouse_pos);
                                println!("{:?}", DisplayInfo::from_point(ui.input(|i| i.pointer.hover_pos().unwrap().x as i32),ui.input(|i| i.pointer.hover_pos().unwrap().y as i32)).unwrap());
                            }

                            if ui.input(|i| {
                                i.pointer.any_down()
                                    && self.mouse_pos.unwrap()[0] == -1.0
                                    && self.mouse_pos.unwrap()[1] == -1.0
                            }) {
                                println!("salvo pressione");
                                self.mouse_pos = ui.input(|i| i.pointer.interact_pos());
                                self.screen_to_show = Some(
                                    DisplayInfo::from_point(
                                        self.mouse_pos.unwrap().x as i32,
                                        self.mouse_pos.unwrap().y as i32,
                                    )
                                    .unwrap()
                                    .id,
                                );
                                // println!("{:?}", self.mouse_pos);
                                // println!("{:?}", DisplayInfo::from_point(self.mouse_pos.unwrap().x as i32,self.mouse_pos.unwrap().y as i32).unwrap());
                            }
                            if self.mouse_pos.unwrap()[0] != -1.0
                                && self.mouse_pos.unwrap()[1] != -1.0
                            {
                                self.mouse_pos_f = ui.input(|i| i.pointer.latest_pos());
                                self.define_rectangle();
                            }
                            if ui.input(|i| i.pointer.any_released()) {
                                frame.set_window_size(Vec2::new(0.0, 0.0));

                                self.selected_window = 3; //Le coordinate sono salvate in self.mouse_pos_2 e self.mouse_posf_2
                            }

                            ui.painter().add(Shape::Rect(RectShape::new(
                                Rect::from_min_max(self.rect_pos, self.rect_pos_f),
                                Rounding::default(),
                                Color32::TRANSPARENT,
                                Stroke::new(2.0, Color32::GRAY),
                            )));
                        });
                }
                ModeOptions::FullScreen => {
                    frame.set_window_size(Vec2::new(0.0, 0.0));
                    self.selected_window = 3;
                }
            }
        } else if self.selected_window == 3 {
            self.selected_window = 4;
        } else if self.selected_window == 4 {
            self.take_screenshot();
            self.selected_window = 5;
        } else if self.selected_window == 5 {
            frame.set_decorations(true);

            if self.width <= 1000.0 && self.height <= 500.0 {
                frame.set_window_size(Vec2::new(1000.0, 500.0)); //1400 750
            } else if self.width <= 1000.0 && self.height >= 500.0 {
                frame.set_window_size(Vec2::new(1000.0, self.height));
            } else if self.width >= 1000.0 && self.height <= 500.0 {
                frame.set_window_size(Vec2::new(self.width, 500.0));
            } else if self.width >= 1200.0 && self.height >= 700.0 {
                frame.set_window_size(Vec2::new(1300.0, 800.0));
            } else {
                frame.set_window_size(Vec2::new(self.width, self.height));
            }

            let mut paint_btn = None;

            let mut text_btn = None;
            let mut save_btn = None;
            let mut save_edit_btn = None;

            egui::CentralPanel::default().show(ctx, |_ui| {
                egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        paint_btn = Some(ui.add(egui::Button::new("🖊 Paint")));
                        if paint_btn.unwrap().clicked() {
                            self.pp_option = Some(PpOptions::Painting);
                            self.selected_shape_string = "Select a shape!".to_string();
                        }
                        egui::ComboBox::from_id_source("Select a shape!")
                            .selected_text(format!("{}", self.selected_shape_string))
                            .show_ui(ui, |ui| {
                                if ui
                                    .selectable_value(
                                        &mut self.selected_shape,
                                        Shapes::Arrow,
                                        "↘ Arrow",
                                    )
                                    .clicked()
                                {
                                    self.selected_shape = Shapes::Arrow;
                                    self.selected_shape_string = "↘ Arrow".to_string();
                                    self.pp_option = Some(PpOptions::Arrow);
                                }

                                if ui
                                    .selectable_value(
                                        &mut self.selected_shape,
                                        Shapes::Circle,
                                        "⭕ Circle",
                                    )
                                    .clicked()
                                {
                                    self.selected_shape = Shapes::Circle;
                                    self.selected_shape_string = "⭕ Circle".to_string();
                                    self.pp_option = Some(PpOptions::Circle);
                                }

                                if ui
                                    .selectable_value(
                                        &mut self.selected_shape,
                                        Shapes::Square,
                                        "⬜ Square",
                                    )
                                    .clicked()
                                {
                                    self.selected_shape = Shapes::Square;
                                    self.selected_shape_string = "⬜ Square".to_string();
                                    self.pp_option = Some(PpOptions::Square);
                                };
                            });
                        text_btn = Some(ui.add(egui::Button::new("✒ Text")));
                        if text_btn.unwrap().clicked() {
                            self.pp_option = Some(PpOptions::Text);
                            self.selected_shape_string = "Select a shape!".to_string();
                        }
                        save_btn = Some(ui.add(egui::Button::new("Save")));
                        save_edit_btn = Some(ui.add(egui::Button::new("Save with name")));
                    });

                    match self.loading_state {
                        LoadingState::Loaded => {
                            println!("fff");
                            let dim: Vec2;
                            if self.width >= 1200.0 && self.height >= 700.0 {
                                dim = Vec2::new(1200.0, 700.0);
                            } else if self.width >= 1200.0 && self.height <= 700.0 {
                                dim = Vec2::new(1200.0, self.height);
                            } else if self.width <= 1200.0 && self.height >= 700.0 {
                                dim = Vec2::new(self.width, 700.0);
                            } else {
                                dim = Vec2::new(self.width, self.height);
                            }

                            let response = self
                                .painting
                                .ui(
                                    ui,
                                    egui::Image::new(self.image.as_ref().unwrap()).shrink_to_fit(),
                                    dim,
                                    self.pp_option.clone().unwrap(),
                                )
                                .clone()
                                .unwrap();

                            if save_btn.unwrap().clicked() {
                                self.image_name = Some(
                                    chrono::offset::Local::now()
                                        .format("%Y-%m-%d_%H_%M_%S")
                                        .to_string(),
                                );

                                let screens = Screen::all().unwrap();
                                let mod_img = screens[0].capture_area(
                                    response.rect.left_top()[0] as i32,
                                    response.rect.left_top()[1] as i32 + 50,
                                    response.rect.width() as u32,
                                    response.rect.height() as u32,
                                );

                                // for screen in screens {
                                //     let mod_img = screen.capture_area(
                                //         response.rect.left_top()[0] as i32,
                                //         response.rect.left_top()[1] as i32 + 50,
                                //         response.rect.width() as u32,
                                //         response.rect.height() as u32,
                                //     );

                                if mod_img.is_err() == false {
                                    if self.current_os == "windows" {
                                        let _ = mod_img.unwrap().save(format!(
                                            "{}\\{}.{}",
                                            self.filepath
                                                .clone()
                                                .unwrap()
                                                .as_os_str()
                                                .to_str()
                                                .unwrap()
                                                .to_string(),
                                            self.image_name.clone().unwrap(),
                                            self.image_format_string
                                        ));
                                    } else {
                                        let _ = mod_img.unwrap().save(format!(
                                            "{}/{}.{}",
                                            self.filepath
                                                .clone()
                                                .unwrap()
                                                .as_os_str()
                                                .to_str()
                                                .unwrap()
                                                .to_string(),
                                            self.image_name.clone().unwrap(),
                                            self.image_format_string
                                        ));
                                    }
                                }
                                //}
                            }
                            if save_edit_btn.unwrap().clicked() {
                                let dialog = FileDialog::new().save_file();

                                let screens = Screen::all().unwrap();
                                let mod_img = screens[0].capture_area(
                                    response.rect.left_top()[0] as i32,
                                    response.rect.left_top()[1] as i32 + 50,
                                    response.rect.width() as u32,
                                    response.rect.height() as u32,
                                );

                                if mod_img.is_err() == false {
                                    let _ = mod_img.unwrap().save(format!(
                                        "{}.{}",
                                        dialog
                                            .clone()
                                            .unwrap()
                                            .as_os_str()
                                            .to_str()
                                            .unwrap()
                                            .to_string(),
                                        self.image_format_string,
                                    ));
                                }
                            }
                        }
                        LoadingState::NotLoaded => {
                            for _i in [0, self.screenshots_taken.len() - 1] {
                                //rimettere -1

                                self.load_image(ui);
                                self.pp_option = Some(PpOptions::Painting);
                                self.loading_state = LoadingState::Loaded;

                                ()
                            }
                        }
                    }
                });
            });
        } else if self.selected_window == 6 {
            egui::CentralPanel::default().show(ctx, |ui| {
                if ui.button("Choose Path").clicked() {
                    self.filepath = FileDialog::new()
                        .set_directory("./screenshot")
                        .pick_folder();
                }

                if ui
                    .add(egui::RadioButton::new(
                        self.image_format == Some(ImageFormat::Jpg),
                        "jpg",
                    ))
                    .clicked()
                {
                    self.image_format = Some(ImageFormat::Jpg);
                    self.image_format_string = "jpg".to_string();
                }
                if ui
                    .add(egui::RadioButton::new(
                        self.image_format == Some(ImageFormat::Png),
                        "png",
                    ))
                    .clicked()
                {
                    self.image_format = Some(ImageFormat::Png);
                    self.image_format_string = "png".to_string();
                }
                if ui
                    .add(egui::RadioButton::new(
                        self.image_format == Some(ImageFormat::Gif),
                        "gif",
                    ))
                    .clicked()
                {
                    self.image_format = Some(ImageFormat::Gif);
                    self.image_format_string = "gif".to_string();
                }
                if ui.button("Exit").clicked() {
                    self.selected_window = 1;
                }
            });
        }
    }
}
