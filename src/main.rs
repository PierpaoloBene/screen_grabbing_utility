use core::fmt;
use std::{fmt::Display, collections::BTreeMap};

use eframe::{
    egui::{self, Layout, RichText, style},
    emath::Align, epaint::FontFamily,
};


#[derive(PartialEq,Debug)]
enum ModeOptions {
    Rectangle,
    FullScreen ,
}

fn main() -> Result<(), eframe::Error> {
    let mut selected:ModeOptions = ModeOptions::Rectangle;
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_simple_native("Screen Grabbing Utility", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(310.0); // da modificare
                if ui.add_sized([50.,50.], egui::Button::new(RichText::new("+").size(30.0))).clicked(){
                    println!("caccona");
                }

                egui::ComboBox::from_id_source("my_combobox").width(200.0)
                .selected_text(RichText::new(format!("{:?}",selected)).size(30.0))
                .show_ui(ui, |ui|{
                    ui.selectable_value(&mut selected,ModeOptions::Rectangle, RichText::new("Rectangle").size(30.0));
                    ui.selectable_value(&mut selected, ModeOptions::FullScreen , RichText::new("FullScreen").size(30.0));
                });
            });
        });
    })
}
