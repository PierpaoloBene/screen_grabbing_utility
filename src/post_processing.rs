use std::f32::consts::PI;

use egui::{emath, vec2, Color32, CursorIcon, Painter, Pos2, Rect, Sense, Stroke, Ui, Vec2};
use image::{GenericImage, Pixel, Rgb, RgbaImage};
use imageproc;

/// Something to view in the demo windows
pub trait View {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        image: egui::Image,
        image_buffer: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        dim: Vec2,
        opt: PpOptions,
    ) -> Option<egui::Response>;
}

/// Something to view
pub trait Demo {
    /// Is the demo enabled for this integraton?
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }

    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    // Show windows, etc
    /*fn show(&mut self, ctx: &egui::Context, open: &mut bool);*/
}
#[derive(Debug, Clone)]
pub enum PpOptions {
    Arrow,
    Circle,
    Square,
    Text,
    Painting,
}
pub struct Painting {
    last_type_added: Vec<PpOptions>,

    
    mult_factor: Option<(f32, f32)>,
    /// in 0-1 normalized coordinates
    lines: Vec<(Vec<Pos2>, Stroke)>,
    lines_stroke: Stroke,
    
    
    starting_point: Pos2,
    final_point: Pos2,
    arrows: Vec<(Pos2, Pos2, Stroke)>,
    arrows_stroke: Stroke,

    circle_center: Pos2,
    radius: f32,
    circles: Vec<(Pos2, f32, Stroke)>,
    circles_stroke: Stroke,
    circles_pixels: Vec<(Vec<Pos2>, Color32)>,

    square_starting_point: Pos2,
    square_ending_point: Pos2,
    squares_stroke: Stroke,
    squares: Vec<(Rect, Stroke)>,
    squares_pixels: Vec<(Vec<Pos2>, Color32)>,
    shift_squares: Option<Pos2>,

    text_starting_position: Pos2,
    text_ending_position: Pos2,
    texts_stroke: Stroke,
    texts: Vec<(String, Pos2, Pos2, Stroke)>,
    to_write_text: String,
    ready_to_write: bool,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            last_type_added: Vec::new(),
            
            mult_factor: None,
            lines: Default::default(),
            lines_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),

           
            

            starting_point: Pos2 { x: -1.0, y: -1.0 },
            final_point: Pos2 { x: -1.0, y: -1.0 },
            arrows: Vec::new(),
            arrows_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),

            circle_center: Pos2 { x: -1.0, y: -1.0 },
            radius: -1.0,
            circles: Vec::new(),
            circles_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
            circles_pixels: Vec::new(),

            square_starting_point: Pos2 { x: -1.0, y: -1.0 },
            square_ending_point: Pos2 { x: -1.0, y: -1.0 },
            squares_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
            squares: Vec::new(),
            squares_pixels: Vec::new(),
            shift_squares: None,

            text_starting_position: Pos2 { x: -1.0, y: -1.0 },
            text_ending_position: Pos2 { x: -1.0, y: -1.0 },
            texts: Vec::new(),
            texts_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
            to_write_text: "Write something".to_string(),
            ready_to_write: false,
        }
    }
}

impl Painting {
    pub fn render_elements(&mut self, painter: Painter, to_screen: emath::RectTransform) {
        if !self.lines.is_empty() {
            let shapes = self
                .lines
                .iter()
                .filter(|line| line.0.len() >= 2)
                .map(|line| {
                    let points: Vec<Pos2> = line.0.iter().map(|p| to_screen * *p).collect();
                    // let (vecio, stroke) = line.clone();
                    // if !self.linea_scritta {
                    //     for posizione in vecio {
                    //         self.lines_pixels.push(posizione);
                    //     }
                    //     self.linea_scritta = true;
                    // }

                    egui::Shape::line(points, line.1)
                });
            painter.extend(shapes);
        }
        if !self.arrows.is_empty() {
            for point in self.arrows.clone().into_iter() {
                painter.arrow(
                    point.0,
                    vec2(point.1.x - point.0.x, point.1.y - point.0.y),
                    point.2,
                );
            }
        }

        if !self.circles.is_empty() {
            for point in self.circles.clone().into_iter() {
                painter.circle(point.0, point.1, egui::Color32::TRANSPARENT, point.2);
                let pixels=self.calc_pixels_circle(point.0, point.1, point.2.width);
                self.circles_pixels.push((pixels, point.2.color));
            }
        }

        if !self.squares.is_empty() || self.squares.len() == 0 {
            for point in self.squares.clone().into_iter() {
                let (rect, stroke) = point;

                let x_min = rect.x_range().min;
                let x_max = rect.x_range().max;

                let y_min = rect.y_range().min;
                let y_max = rect.y_range().max;

                let pixels=self.calc_pixels_rect(
                    Pos2::new(x_min, y_min),
                    Pos2::new(x_max, y_max),
                    stroke.width,
                );
                self.squares_pixels.push((pixels, point.1.color));

                painter.rect(point.0, 1.0, egui::Color32::TRANSPARENT, point.1);
            }
        }

        if !self.texts.is_empty() {
            for point in self.texts.clone().into_iter() {
                painter.text(
                    point.1,
                    egui::Align2::LEFT_TOP,
                    point.0,
                    egui::FontId::monospace(15.0),
                    point.3.color,
                );
            }
        }
    }
    fn undo(&mut self) {
        match self.last_type_added.last().unwrap() {
            PpOptions::Arrow => {
                self.arrows.remove(self.arrows.len() - 1);
            }
            PpOptions::Circle => {
                self.circles.remove(self.circles.len() - 1);
            }
            PpOptions::Square => {
                self.squares.remove(self.squares.len() - 1);
            }
            PpOptions::Text => {
                self.texts.remove(self.texts.len() - 1);
            }
            _ => {}
        }
        self.last_type_added.pop();
    }

    pub fn ui_control(&mut self, ui: &mut egui::Ui, opt: PpOptions, image_buffer: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> egui::Response {
        println!("In ui_control");
        let mut res = None;
        match opt {
            PpOptions::Painting => {
                if self.lines.last_mut() == None {
                    res = Some(
                        ui.horizontal(|ui| {
                            egui::stroke_ui(ui, &mut self.lines_stroke, "Stroke");
                            ui.separator();
                            if ui.button("Clear Painting").clicked() {
                                self.lines.clear();
                            }
                        })
                        .response,
                    );
                    res.unwrap()
                } else {
                    let res = ui
                        .horizontal(|ui| {
                            egui::stroke_ui(ui, &mut self.lines.last_mut().unwrap().1, "Stroke");
                            ui.separator();
                            if ui.button("Clear Painting").clicked() {
                                self.lines.clear();
                            }
                        })
                        .response;
                    if !self.lines.is_empty() {
                        self.lines_stroke = self.lines.last_mut().unwrap().1;
                    }

                    res
                }
            }
            PpOptions::Arrow => {
                let mut back_btn = None;
                ui.horizontal(|ui| {
                    egui::stroke_ui(ui, &mut self.arrows_stroke, "Stroke");
                    ui.separator();
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add(egui::Button::new("Undo")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    }
                })
                .response
            }
            PpOptions::Circle => {
                println!("In ui_control circles");
                let mut back_btn = None;
                let mut save_btn=None;
                ui.horizontal(|ui| {
                    egui::stroke_ui(ui, &mut self.circles_stroke, "Stroke");
                    ui.separator();
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add(egui::Button::new("Undo")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    }
                    
                    save_btn=Some(ui.add(egui::Button::new("Finish editing")));
                    if save_btn.unwrap().clicked() && !self.circles_pixels.is_empty(){
                        for p in self.circles_pixels.clone() {
                            for pi in p.0{
                                let image_pixel = image_buffer.get_pixel_mut(pi.x as u32, pi.y as u32);
                                
                                *image_pixel = image::Rgba([p.1.r(), p.1.g(), p.1.b(), p.1.a()]);
                            }
                            
                        }
                    }
                    

                })
                .response
            }
            PpOptions::Square => {
                println!("In ui_control squares");
                let mut back_btn = None;
                let mut save_btn=None;
                ui.horizontal(|ui: &mut Ui| {
                    egui::stroke_ui(ui, &mut self.squares_stroke, "Stroke");
                    ui.separator();
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add(egui::Button::new("Undo")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    }
                    save_btn=Some(ui.add(egui::Button::new("Finish editing")));
                    if save_btn.unwrap().clicked() && !self.circles_pixels.is_empty(){
                        for p in self.squares_pixels.clone() {
                            for pi in p.0{
                                let image_pixel = image_buffer.get_pixel_mut(pi.x as u32, pi.y as u32);
                
                                *image_pixel = image::Rgba([p.1.r(), p.1.g(), p.1.b(), p.1.a()]);
                
                            }
                            
                        }
                    }
                })
                .response
            }
            PpOptions::Text => {
                println!("In ui_control texts");
                let mut write_btn = None;
                let mut back_btn = None;
                ui.horizontal(|ui: &mut Ui| {
                    egui::stroke_ui(ui, &mut self.texts_stroke, "Stroke");
                    ui.separator();
                    ui.add(egui::TextEdit::singleline(&mut self.to_write_text));
                    ui.separator();
                    write_btn = Some(ui.add(egui::Button::new("Write!")));
                    if write_btn.unwrap().clicked()
                        && self.text_starting_position.x != -1.0
                        && self.text_starting_position.y != -1.0
                        && self.text_ending_position.x != -1.0
                        && self.text_ending_position.y != -1.0
                    {
                        self.to_write_text = self.to_write_text.clone();
                        self.ready_to_write = true;
                    }
                    if self.last_type_added.len() > 0 {
                        back_btn = Some(ui.add(egui::Button::new("Undo")));
                        if back_btn.unwrap().clicked() {
                            self.undo();
                        }
                    }
                })
                .response
            }
            _ => res.unwrap(),
        }
    }

    pub fn ui_content(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        image_buffer: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        dim: Vec2,
    ) -> egui::Response {
        println!("In ui_content");

        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );

        image.paint_at(ui, response.rect);
        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if (mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y))
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }

        self.render_elements(painter.clone(), to_screen);

        let from_screen = to_screen.inverse();

        if self.lines.is_empty() {
            self.lines.push((vec![], self.lines_stroke));
        }

        let mut current_line = &mut self.lines.last_mut().unwrap().0;

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push((vec![], self.lines_stroke));
            response.mark_changed();
        }

        self.render_elements(painter.clone(), to_screen);

        response
    }

    pub fn ui_content_arrows(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        dim: Vec2,
    ) -> egui::Response {
        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        image.paint_at(ui, response.rect);

        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if (mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y))
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }
        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) {
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.starting_point.x == -1.0
                && self.starting_point.y == -1.0
            {
                self.starting_point = pos.unwrap();
            }
        }

        if ui.input(|i| i.pointer.any_released()) {
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.final_point.x == -1.0
                && self.final_point.y == -1.0
                && self.starting_point.x != -1.0
                && self.starting_point.y != -1.0
            {
                self.final_point = pos.unwrap();
            }
        }

        if self.final_point.x != -1.0
            && self.final_point.y != -1.0
            && self.starting_point.x != -1.0
            && self.starting_point.y != -1.0
        {
            self.arrows
                .push((self.starting_point, self.final_point, self.arrows_stroke));
            self.starting_point = Pos2 { x: -1.0, y: -1.0 };
            self.final_point = Pos2 { x: -1.0, y: -1.0 };
            self.last_type_added.push(PpOptions::Arrow);
        }

        self.render_elements(painter.clone(), to_screen);

        response
    }

    pub fn ui_content_circles(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        image_buffer: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        dim: Vec2,
    ) -> egui::Response {
        println!("In ui_content circles");

        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        self.mult_factor = Some((
            image_buffer.width() as f32 / response.rect.width(),
            image_buffer.height() as f32 / response.rect.height(),
        ));
        image.paint_at(ui, response.rect);
        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if (mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y))
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }
        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if (mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y))
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }
        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) {
            let pos = ui.input(|i| i.pointer.latest_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.circle_center.x == -1.0
                && self.circle_center.y == -1.0
            {
                self.circle_center = ui.input(|i| i.pointer.interact_pos().unwrap());
            }
        }

        if ui.input(|i| i.pointer.any_released())
            && self.circle_center.x != -1.0
            && self.circle_center.y != -1.0
            && self.radius == -1.0
        {
            self.radius = ui.input(|i| i.pointer.interact_pos().unwrap()).x - self.circle_center.x;
            self.radius = self.radius.abs();
        }

        if self.circle_center.x != -1.0 && self.circle_center.y != -1.0 && self.radius != -1.0 {
            self.circles
                .push((self.circle_center, self.radius, self.circles_stroke));
            self.circle_center = Pos2 { x: -1.0, y: -1.0 };
            self.radius = -1.0;
            self.last_type_added.push(PpOptions::Circle);
            self.shift_squares = Some(Pos2::new(
                response.rect.left_top().x,
                response.rect.left_top().y,
            ));
        }
        self.render_elements(painter.clone(), to_screen);

        


        response
    }

    pub fn ui_content_squares(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        image_buffer: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        dim: Vec2,
    ) -> egui::Response {
        println!("In ui_content squares");

        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        self.mult_factor = Some((
            image_buffer.width() as f32 / response.rect.width(),
            image_buffer.height() as f32 / response.rect.height(),
        ));

        image.paint_at(ui, response.rect);
        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if (mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y))
        {
            ui.ctx()
                .output_mut(|i| i.cursor_icon = CursorIcon::Crosshair);
        }
        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) {
            let pos = response.interact_pointer_pos();
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.square_starting_point.x == -1.0
                && self.square_starting_point.y == -1.0
            {
                self.shift_squares = Some(Pos2::new(
                    response.rect.left_top().x,
                    response.rect.left_top().y,
                ));
                self.square_starting_point = pos.unwrap();
            }
        }

        if ui.input(|i| i.pointer.any_released()) {
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.square_ending_point.x == -1.0
                && self.square_ending_point.y == -1.0
                && self.square_starting_point.x != -1.0
                && self.square_starting_point.y != -1.0
            {
                self.square_ending_point = pos.unwrap();
            }
        }

        if self.square_starting_point.x != -1.0
            && self.square_starting_point.y != -1.0
            && self.square_ending_point.x != -1.0
            && self.square_ending_point.y != 1.0
        {
            let re =
                egui::Rect::from_points(&[self.square_starting_point, self.square_ending_point]);

            self.squares.push((re, self.squares_stroke));
            self.square_starting_point.x = -1.0;
            self.square_starting_point.y = -1.0;
            self.square_ending_point.x = -1.0;
            self.square_ending_point.y = -1.0;
            self.last_type_added.push(PpOptions::Square);
        }

        self.render_elements(painter.clone(), to_screen);

        

        response
    }

    pub fn ui_content_texts(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        dim: Vec2,
    ) -> egui::Response {
        println!("In ui_content texts");

        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        image.paint_at(ui, response.rect);
        let mouse_pos = ui.input(|i| i.pointer.interact_pos());
        if (mouse_pos.is_none() == false
            && response.rect.x_range().contains(mouse_pos.unwrap().x)
            && response.rect.y_range().contains(mouse_pos.unwrap().y))
        {
            ui.ctx().output_mut(|i| i.cursor_icon = CursorIcon::Text);
        }
        self.render_elements(painter.clone(), to_screen);

        if ui.input(|i| i.pointer.any_pressed()) {
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.text_starting_position.x == -1.0
                && self.text_starting_position.y == -1.0
            {
                self.text_starting_position = pos.unwrap();
            }
        }

        if ui.input(|i| i.pointer.any_released()) {
            let pos = ui.input(|i| i.pointer.interact_pos());
            if pos.is_none() == false
                && response.rect.contains(pos.unwrap())
                && self.text_ending_position.x == -1.0
                && self.text_ending_position.y == -1.0
                && self.text_starting_position.x != -1.0
                && self.text_starting_position.y != -1.0
            {
                self.text_ending_position = pos.unwrap();
            }
        }

        if self.text_starting_position.x != -1.0
            && self.text_starting_position.y != -1.0
            && self.text_ending_position.x != -1.0
            && self.text_ending_position.y != -1.0
        {
            self.render_elements(painter.clone(), to_screen);
            if self.ready_to_write {
                self.texts.push((
                    self.to_write_text.clone(),
                    self.text_starting_position,
                    self.text_ending_position,
                    self.texts_stroke,
                ));

                self.text_starting_position.x = -1.0;
                self.text_starting_position.y = -1.0;
                self.text_ending_position.x = -1.0;
                self.text_ending_position.y = -1.0;
                self.ready_to_write = false;
                self.last_type_added.push(PpOptions::Text);
            }
        }

        self.render_elements(painter.clone(), to_screen);

        response
    }

    pub fn calc_pixels_rect(&mut self, start: Pos2, end: Pos2, thickness: f32) -> Vec<Pos2> {
        let mut pixels: Vec<Pos2> = Vec::new();

        let min_x = start.x.min(end.x) - self.shift_squares.unwrap().x;
        let max_x =
            start.x.max(end.x) - self.shift_squares.unwrap().x * self.mult_factor.unwrap().0;
        let min_y = start.y.min(end.y) - self.shift_squares.unwrap().y;
        let max_y =
            start.y.max(end.y) - self.shift_squares.unwrap().y * self.mult_factor.unwrap().1;

        // Calcolo delle posizioni del contorno orizzontale superiore e inferiore
        for x in
            (min_x as i32 - (thickness / 2.0) as i32)..=(max_x as i32 + (thickness / 2.0) as i32)
        {
            for i in 0..=((thickness / 2.0) as i32) {
                pixels.push(Pos2 {
                    x: x as f32,
                    y: min_y - i as f32,
                });
                pixels.push(Pos2 {
                    x: x as f32,
                    y: max_y + i as f32,
                });
            }
        }

        // Calcolo delle posizioni del contorno verticale sinistro e destro
        for y in ((min_y as i32)..=(max_y as i32)) {
            for i in 0..=((thickness.ceil() / 2.0) as i32) {
                pixels.push(Pos2 {
                    x: min_x - i as f32,
                    y: y as f32,
                });
                pixels.push(Pos2 {
                    x: max_x + i as f32,
                    y: y as f32,
                });
            }
        }

        pixels
    }

    pub fn calc_pixels_circle(&mut self, center: Pos2, radius: f32, thickness: f32) -> Vec<Pos2> {
        let mut points = Vec::new();
    
        let num_points = 10000; // Numero di punti per approssimare la circonferenza
        //thickness=100.0;
        let center_x=center.x-self.shift_squares.unwrap().x * self.mult_factor.unwrap().0;
        let center_y=center.y-self.shift_squares.unwrap().y *self.mult_factor.unwrap().1;
        for t in 0..(thickness*1.5) as i32{
            for i in 0..num_points {
                let angle = 2.0 * PI * (i as f32) / (num_points as f32);
                let x = center_x + (radius - t as f32/ 2.0) * angle.cos();
                let y = center_y + (radius - t as f32 / 2.0) * angle.sin();
                let point = Pos2 { x, y };
                points.push(point);
            }
        }
        
    
        points
    }
}

impl Demo for Painting {
    fn name(&self) -> &'static str {
        "🖊 Painting"
    }
}

impl View for Painting {
    fn ui(
        &mut self,
        ui: &mut Ui,
        image: egui::widgets::Image,
        image_buffer: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        dim: Vec2,
        opt: PpOptions,
    ) -> Option<egui::Response> {
        let mut resp = None;

        match opt {
            PpOptions::Painting => {
                self.ui_control(ui, opt, image_buffer);
                ui.label("Paint with your mouse/touch!");
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        resp = Some(self.ui_content(ui, image, image_buffer, dim));
                    });
                });
            }
            PpOptions::Arrow => {
                self.ui_control(ui, opt,image_buffer);
                ui.label("Paint an arrow with your mouse/touch!");
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        resp = Some(self.ui_content_arrows(ui, image, dim));
                    });
                });
            }
            PpOptions::Circle => {
                self.ui_control(ui, opt,image_buffer);
                ui.label("Paint a circle with your mouse/touch!");
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        resp = Some(self.ui_content_circles(ui, image, image_buffer, dim));
                    });
                });
            }
            PpOptions::Square => {
                self.ui_control(ui, opt, image_buffer);
                ui.label("Paint a square with your mouse/touch!");
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        resp = Some(self.ui_content_squares(ui, image, image_buffer, dim));
                    });
                });
            }
            PpOptions::Text => {
                self.ui_control(ui, opt, image_buffer);
                ui.label("First, click were you want to write and then write something!");
                ui.vertical_centered(|ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        resp = Some(self.ui_content_texts(ui, image, dim));
                    });
                });
            }
        }

        resp
    }
}
