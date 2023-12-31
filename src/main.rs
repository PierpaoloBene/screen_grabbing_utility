// mod post_processing;
// use crate::post_processing::PpOptions;
// use crate::post_processing::View;
mod pp_no_stroke;
use crate::pp_no_stroke::PpOptions;
use crate::pp_no_stroke::View;
use chrono;
use eframe::glow::PRIMITIVE_RESTART_INDEX;
use egui::ColorImage;
use egui::CursorIcon;
use egui::FontImage;
use egui::Image;
use egui::ImageData;
use egui::Response;
use egui::Rgba;
use egui::Style;
use egui::TextBuffer;
use egui_notify::Toast;
use image::DynamicImage;
use image::EncodableLayout;
use image::ImageBuffer;
use image::Rgb;
use image::imageops::crop;
use imageproc;
use imageproc::drawing::draw_line_segment;
use rfd::FileDialog;
use rusttype::Font;
use arboard::Clipboard;
use egui_notify::Toasts;
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
    //let p = post_processing::Painting::default();
    let p=pp_no_stroke::Painting::default();
    let mut toasts=Toasts::default();
    
    manager.register(hotkey_exit).unwrap();
    manager.register(hotkey_screen).unwrap();
   let openfw = GlobalHotKeyEvent::receiver();
    eframe::run_native(
        "Screen Grabbing Utility",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(FirstWindow {
                toasts:Some(Toasts::default()),
                show_toast:false,
                number_of_screens:None,
                screen_to_show: None,
                frame_initial_pos:None,
                image_name: None,
                image_format: Some(ImageFormat::Jpg),
                image_format_string: "jpg".to_string(),
                pp_option: None,
                current_os: current_os.to_string(),
                multiplication_factor: None,
                screen_size: None,
                loading_state: LoadingState::NotLoaded,
                image: None,
                image_texture: None,
                image_buffer: None,
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
                mult_factor: None,
                cut_clicked: false,
                cropped:false,
                circle_pixels: Vec::new(),
                square_pixels: Vec::new(),
                arrow_pixels: Vec::new(),
                text_pixels: Vec::new(),
                line_pixels: Vec::new(),
                save:false,
                to_cut_rect:None,
                shrink_fact:None,
            })
        }),
    )
}

struct FirstWindow {
    toasts:Option<Toasts>,
    show_toast:bool,
    number_of_screens:Option<usize>,
    screen_to_show: Option<u32>,
    frame_initial_pos:Option<Pos2>,
    image_name: Option<String>,
    image_format: Option<ImageFormat>,
    image_format_string: String,
    pp_option: Option<PpOptions>,
    current_os: String,
    multiplication_factor: Option<f32>,
    screen_size: Option<Vec2>,
    loading_state: LoadingState,
    image: Option<TextureHandle>,
    image_texture: Option<egui::ColorImage>,
    image_buffer: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
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
    //painting: post_processing::Painting,
    painting: pp_no_stroke::Painting,
    width: f32,
    height: f32,
    mult_factor: Option<(f32, f32)>,
    cut_clicked: bool,
    cropped:bool,
    circle_pixels: Vec<(Pos2, f32, Color32)>,
    square_pixels: Vec<(Rect, Color32)>,
    arrow_pixels: Vec<(Vec<Pos2>, Color32)>,
    text_pixels: Vec<(Pos2, Color32, String)>,
    line_pixels: Vec<(Vec<Pos2>, Color32)>,
    save:bool,
    to_cut_rect:Option<(Pos2, Pos2)>,
    shrink_fact:Option<f32>,
    
}

impl eframe::App for FirstWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        
        let screens=Screen::all().unwrap();
        if self.screen_to_show.is_none(){
            self.screen_to_show=Some(screens[0].display_info.id);
            self.screen_size=Some(Vec2::new(screens[0].display_info.width as f32, screens[0].display_info.height as f32));
            self.frame_initial_pos=Some(Pos2::new(screens[0].display_info.x as f32, screens[0].display_info.y as f32));
        }
        self.number_of_screens=Some(screens.len());
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
                        .add_sized([50., 50.], egui::Button::new(RichText::new("+").size(30.0)))
                        .on_hover_text("Ctrl+D")
                        .clicked()
                    {
                        println!("premuto +");
                        
                        std::thread::sleep(Duration::from_secs(self.selected_timer_numeric));
                        //self.screen_size = frame.info().clone().window_info.monitor_size; 
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

           // println!("{:?} {:?}", self.screen_size, self.frame_initial_pos);

            frame.set_window_size(self.screen_size.unwrap());
            

            frame.set_window_pos(self.frame_initial_pos.unwrap());
            self.multiplication_factor=frame.info().native_pixels_per_point;
                        
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

                            if ui.input(|i| {
                                i.pointer.any_down()
                                    && self.mouse_pos.unwrap()[0] == -1.0
                                    && self.mouse_pos.unwrap()[1] == -1.0
                            }) {
                                println!("salvo pressione");
                                self.mouse_pos = ui.input(|i| i.pointer.interact_pos());
                                // self.screen_to_show = Some(
                                //     DisplayInfo::from_point(
                                //         self.mouse_pos.unwrap().x as i32,
                                //         self.mouse_pos.unwrap().y as i32,
                                //     )
                                //     .unwrap()
                                //     .id,
                                // );
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
            frame.set_window_pos(Pos2{x: 0.0, y: 0.0});
            
            println!("w={:} , h={:}",self.width,self.height);
           
                
               
            if self.width <= 1000.0 && self.height <= 500.0 {
                frame.set_window_size(Vec2::new(1100.0, 600.0)); //1400 750
                println!("1");
            } else if self.width <= 1000.0 && self.height >= 500.0 {
                frame.set_window_size(Vec2::new(1100.0, self.height+self.height*0.3));
                println!("2");
            } else if self.width >= 1000.0 && self.height <= 500.0 {
                frame.set_window_size(Vec2::new(self.screen_size.unwrap().x /self.multiplication_factor.unwrap(), 600.0));
                println!("3");
            } else if self.width >= 1200.0 && self.height >= 700.0 {
                println!("4");
                frame.set_window_size(Vec2::new(1300.0, 800.0));
            } else {
                println!("5");
                frame.set_window_size(Vec2::new(self.screen_size.unwrap().x /self.multiplication_factor.unwrap()- self.screen_size.unwrap().x /self.multiplication_factor.unwrap()*0.001, self.screen_size.unwrap().y /self.multiplication_factor.unwrap()- self.screen_size.unwrap().y /self.multiplication_factor.unwrap()*0.01));
            }

           

            let mut paint_btn = None;

            let mut text_btn = None;
            let mut save_btn = None;
            let mut save_edit_btn = None;
            let mut copy_btn=None;
            let mut crop_btn=None;
            let mut finish_crop=None;
            if self.show_toast{
                // self.toasts.dismiss_oldest_toast();
                // self.toasts.info(format!("Image saved at {:?}", self.filepath)).set_duration(Some(Duration::from_secs(5)));
                self.toasts.as_mut().unwrap().show(ctx); 
                    
            }
            
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
                        copy_btn = Some(ui.add(egui::Button::new("Copy")));
                        crop_btn=Some(ui.add(egui::Button::new("Cut")));
                        finish_crop=Some(ui.add(egui::Button::new("Finish Your Cut")));
                    });
                   
                    match self.loading_state {                        
                        LoadingState::Loaded => {
                            let dim: Vec2;
                            if self.width >= 1200.0 && self.height >= 700.0 {
                                self.shrink_fact=Some(0.7);
                                dim = Vec2::new(self.width*0.7, self.height*0.7); 
                            } else if self.width >= 1200.0 && self.height <= 700.0 {
                                self.shrink_fact=Some(0.7);
                                dim = Vec2::new(self.width*0.7, self.height*0.7);
                            } else if self.width <= 1200.0 && self.height >= 700.0 {                                                        
                               self.shrink_fact=Some(0.6);
                                dim = Vec2::new(self.width*0.6 , self.height*0.6);
                            } else {
                                self.shrink_fact=Some(1.0);
                                dim = Vec2::new(self.width, self.height);
                            }
                            let mut pxs = None;
                            let mut arr=None;
                            let mut txt = None;
                            let mut sqrs = None;
                            let mut crcls=None;
                            let mut response=None;
                            
                            

                            (pxs, arr, txt, sqrs, crcls,response) = self
                                .painting
                                .ui(
                                    ui,
                                    egui::Image::new(self.image.as_ref().unwrap()).shrink_to_fit(),
                                    &mut self.mult_factor,
                                    dim,
                                    self.pp_option.clone().unwrap(),
                                    self.save,
                                    self.cut_clicked,
                                )
                                .clone();
                                
                                  
                                self.save=false;
                                self.cropped=false;
                                match self.pp_option.clone().unwrap() {
                                    PpOptions::Painting => {
                                        if pxs.is_none() == false {
                                            self.line_pixels = pxs.clone().unwrap();
                                        }
                                    }
                                    PpOptions::Arrow => {
                                        if arr.is_none() == false {
                                           
                                        self.arrow_pixels=arr.clone().unwrap();
                                            
                                        }
                                    }
                                    PpOptions::Circle => {
                                        if crcls.is_none() == false {
                                            self.circle_pixels = crcls.clone().unwrap();
                                        }
                                    }
                                    PpOptions::Square => {
                                        if sqrs.is_none() == false {
                                            self.square_pixels = sqrs.clone().unwrap();
                                        }
                                    }
                                    PpOptions::Text => {
                                        if txt.is_none() == false {
                                            self.text_pixels=txt.clone().unwrap();
                                        }
                                    }
                                }

                            if save_btn.unwrap().clicked() {
                                self.save=true;
                               
                                self.image_name = Some(
                                    chrono::offset::Local::now()
                                        .format("%Y-%m-%d_%H_%M_%S")
                                        .to_string(),
                                );
                                self.toasts.as_mut().unwrap().success(format!("Image saved in ./screenshot/{}",self.image_name.clone().unwrap())).set_duration(Some(Duration::from_secs(5)));
                                
                                self.show_toast=true;
                                self.edit_image(ui);

                                let screens = Screen::all().unwrap();
                                
                                let mod_img = self.image_buffer.clone();

                                // for screen in screens {
                                //     let mod_img = screen.capture_area(
                                //         response.rect.left_top()[0] as i32,
                                //         response.rect.left_top()[1] as i32 + 50,
                                //         response.rect.width() as u32,
                                //         response.rect.height() as u32,
                                //     );

                                if mod_img.is_none() == false {
                                    
            
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
                                let dialog = FileDialog::new().add_filter(".jpg", &["jpg"]).add_filter(".png", &["png"]).add_filter(".gif", &["gif"]).save_file();
                                println!("{:?}", dialog);
                                self.save=true;
                                
                                self.toasts.as_mut().unwrap().success(format!("Image saved in {}",dialog.clone().unwrap().to_str().unwrap())).set_duration(Some(Duration::from_secs(5)));
                                
                                self.show_toast=true;
                                
                                self.edit_image(ui);
                                let mod_img = self.image_buffer.clone();
                                if mod_img.is_none() == false {
                                    let _ = mod_img.unwrap().save(format!(
                                        "{}",
                                        dialog
                                            .clone()
                                            .unwrap()
                                            .as_os_str()
                                            .to_str()
                                            .unwrap()
                                            .to_string(),
                                       
                                    ));
                                }
                            }
                            if copy_btn.unwrap().clicked(){
                                self.edit_image(ui);
                                self.toasts.as_mut().unwrap().success("Image copied to clipboard" ).set_duration(Some(Duration::from_secs(5)));
                                
                                self.show_toast=true;
                                let mut clipboard = Clipboard::new().unwrap();
                                let w=self.image_buffer.clone().unwrap().width() as usize;
                                let h=self.image_buffer.clone().unwrap().height() as usize;clipboard.set_image(arboard::ImageData { width: w, height: h, bytes: self.image_buffer.clone().unwrap().as_bytes().into()});
                            }

                            if (crop_btn.unwrap().clicked() || self.cut_clicked==true){
                                self.cut_clicked=true;
                                if self.arrow_pixels.len()>0
                                    || self.circle_pixels.len()>0
                                    || self.square_pixels.len()>0
                                    || self.text_pixels.len()>0
                                    || self.line_pixels.len()>0{
                                        self.edit_image(ui);
                                    }
                                
                                let mut pos_bug_fixed=Pos2::new(0.0,0.0);

                                if ui.input(|i| i.pointer.hover_pos().is_none()==false){
                                    
                                    ui.input(|i| 
                                        pos_bug_fixed=i.pointer.hover_pos().unwrap()
                                        );
                                }

                                
                                if   pos_bug_fixed.x<=response.clone().unwrap().rect.right_top().x &&
                                     pos_bug_fixed.x>=response.clone().unwrap().rect.left_top().x &&
                                     pos_bug_fixed.y>=response.clone().unwrap().rect.left_top().y &&
                                     pos_bug_fixed.y<=response.clone().unwrap().rect.right_bottom().y {
                                
                            egui::Window::new("precut")
                            .constraint_to(response.clone().unwrap().rect)
                            .default_width(dim[0]-0.0)//da modificare
                            .default_height(dim[1]-0.0)//da modificare
                            .title_bar(false)
                            .vscroll(false)
                            .interactable(true)
                            .resizable(false)
                            .frame(egui::Frame::none()
                                     .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 0))
                                     )
                            .show(ctx, |ui|{
                                ui.allocate_space(ui.available_size());
                                
                                egui::Window::new("cut")
                                .constraint_to(response.clone().unwrap().rect)
                                .default_width(dim[0]-1.0)//da modificare
                                .default_height(dim[1]-1.0)//da modificare
                                .title_bar(false)
                                .default_pos(Pos2::new(response.clone().unwrap().rect.left_top().x+1.0, response.clone().unwrap().rect.left_top().y+1.0))
                                .vscroll(false)
                                .interactable(true)
                                .resizable(true)
                                .frame(egui::Frame::none()
                                     .fill(egui::Color32::from_rgba_unmultiplied(70, 70, 70, 70))
                                     .stroke(Stroke::new(1.0, egui::Color32::WHITE))
                                     )
                                .show(ctx, |ui| {
                                     //2 linee verticali
                                     
                                     ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.33, ui.available_rect_before_wrap().left_top().y),
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.33, ui.available_rect_before_wrap().right_bottom().y)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));

                                    ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.66, ui.available_rect_before_wrap().left_top().y),
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.66, ui.available_rect_before_wrap().right_bottom().y)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));

                                    //2 linee orizzontali
                                    ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.33),
                                            Pos2::new(ui.available_rect_before_wrap().right_bottom().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.33)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));

                                    ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.66),
                                            Pos2::new(ui.available_rect_before_wrap().right_bottom().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.66)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));
                                    //println!("pos_left_top_corner:{:},{:}  , pos_left_bottom_corner:{:},{:} pos_right_top_corner={:?} pos_right_bottom_corner={:?} ",ui.available_rect_before_wrap().left_top().x,ui.available_rect_before_wrap().left_top().y,ui.available_rect_before_wrap().left_bottom().x,ui.available_rect_before_wrap().left_bottom().y, ui.available_size_before_wrap(), ui.available_size_before_wrap());
                                    self.to_cut_rect= Some((ui.available_rect_before_wrap().left_top(), ui.available_rect_before_wrap().right_bottom()));
                                    
                                    ui.allocate_space(ui.available_size());
                                    
                                });

                            });
                        }else{
                            
                            egui::Window::new("precut")
                            .constraint_to(response.clone().unwrap().rect)
                            .default_width(dim[0]-0.0)//da modificare
                            .default_height(dim[1]-0.0)//da modificare
                            .title_bar(false)
                            .vscroll(false)
                            .interactable(false)
                            .resizable(false)
                            .frame(egui::Frame::none()
                                     .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 0))
                                     )
                            .show(ctx, |ui|{
                                ui.allocate_space(ui.available_size());
                                
                                egui::Window::new("cut")
                                .constraint_to(response.clone().unwrap().rect)
                                .default_width(dim[0]-1.0)//da modificare
                                .default_height(dim[1]-1.0)//da modificare
                                .title_bar(false)
                                .default_pos(Pos2::new(response.clone().unwrap().rect.left_top().x+1.0, response.clone().unwrap().rect.left_top().y+1.0))
                                .vscroll(false)
                                .resizable(false)
                                .interactable(false)
                                .frame(egui::Frame::none()
                                     .fill(egui::Color32::from_rgba_unmultiplied(70, 70, 70, 70))
                                     .stroke(Stroke::new(1.0, egui::Color32::WHITE))
                                     )
                                .show(ctx, |ui| {
                                     //2 linee verticali
                                     
                                     ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.33, ui.available_rect_before_wrap().left_top().y),
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.33, ui.available_rect_before_wrap().right_bottom().y)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));

                                    ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.66, ui.available_rect_before_wrap().left_top().y),
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x+(ui.available_rect_before_wrap().right_bottom().x-ui.available_rect_before_wrap().left_top().x)*0.66, ui.available_rect_before_wrap().right_bottom().y)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));

                                    //2 linee orizzontali
                                    ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.33),
                                            Pos2::new(ui.available_rect_before_wrap().right_bottom().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.33)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));

                                    ui.painter().add(
                                        egui::Shape::dashed_line(
                                        &[
                                            Pos2::new(ui.available_rect_before_wrap().left_top().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.66),
                                            Pos2::new(ui.available_rect_before_wrap().right_bottom().x,ui.available_rect_before_wrap().left_top().y+(ui.available_rect_before_wrap().right_bottom().y-ui.available_rect_before_wrap().left_top().y)*0.66)],
                                        Stroke::new(2.0, Color32::WHITE),
                                        10.0, 5.0));
                                    //println!("pos_left_top_corner:{:},{:}  , pos_left_bottom_corner:{:},{:} pos_right_top_corner={:?} pos_right_bottom_corner={:?} ",ui.available_rect_before_wrap().left_top().x,ui.available_rect_before_wrap().left_top().y,ui.available_rect_before_wrap().left_bottom().x,ui.available_rect_before_wrap().left_bottom().y, ui.available_size_before_wrap(), ui.available_size_before_wrap());
                                    self.to_cut_rect= Some((ui.available_rect_before_wrap().left_top(), ui.available_rect_before_wrap().right_bottom()));
                                    
                                    ui.allocate_space(ui.available_size());
                                    
                                });

                            });

                        }

                                if finish_crop.unwrap().clicked(){

                                    self.cut_clicked=false;
                                    self.load_cutted_img(ui, response);
                                    self.cropped=true;
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
            let screens=Screen::all().unwrap();
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
                ui.add(egui::Label::new("Select a monitor:"));
                if ui
                    .add(egui::RadioButton::new(
                        self.screen_to_show==Some(screens[0].display_info.id),
                        "Primary",
                    ))
                    .clicked()
                {
                    self.screen_to_show=Some(screens[0].display_info.id);
                    self.screen_size=Some(Vec2::new(screens[0].display_info.width as f32, screens[0].display_info.height as f32));
                    self.frame_initial_pos=Some(Pos2::new(screens[0].display_info.x as f32, screens[0].display_info.y as f32));
                    println!("{:?}", screens[0].display_info.scale_factor);
                }
                if screens.len()==2{
                    if ui
                    .add(egui::RadioButton::new(
                        self.screen_to_show==Some(screens[1].display_info.id),
                        "Secondary",
                    ))
                    .clicked()
                {
                    self.screen_to_show=Some(screens[1].display_info.id);
                    self.screen_size=Some(Vec2::new(screens[1].display_info.width as f32, screens[1].display_info.height as f32));
                    self.frame_initial_pos=Some(Pos2::new(screens[1].display_info.x as f32, screens[1].display_info.y as f32));
                   
                }
                }
               

                if ui.button("Exit").clicked() {
                    self.selected_window = 1;
                }

            });
        }
    }
}
