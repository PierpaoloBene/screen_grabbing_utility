use egui::{emath, vec2, Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2, Painter};

/// Something to view in the demo windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, image: egui::Image, dim: Vec2) -> Option<egui::Response>;
    fn ui_arrows(
        &mut self,
        ui: &mut egui::Ui,
        image: egui::Image,
        dim: Vec2,
    ) -> Option<egui::Response>;

    fn ui_circles(&mut self, ui: &mut egui::Ui, image: egui::Image, dim: Vec2) -> Option<egui::Response>;
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
pub struct Painting {
    /// in 0-1 normalized coordinates
    lines: Vec<(Vec<Pos2>, Stroke)>,
    lines_stroke: Stroke,

    starting_point: Pos2,
    final_point: Pos2,
    arrows: Vec<(Pos2, Pos2, Stroke)>,
    arrows_stroke:Stroke,

    circle_center:Pos2,
    radius:f32,
    circles: Vec<(Pos2, f32, Stroke)>,
    circles_stroke:Stroke
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            lines_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),

            starting_point: Pos2 { x: -1.0, y: -1.0 },
            final_point: Pos2 { x: -1.0, y: -1.0 },
            arrows: Vec::new(),
            arrows_stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),

            circle_center:Pos2 { x: -1.0, y: -1.0 },
            radius:-1.0,
            circles: Vec::new(),
            circles_stroke:Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
        }
    }
}

impl Painting {
    pub fn render_elements(&mut self, painter:Painter){
        if !self.arrows.is_empty() {
            for point in self.arrows.clone().into_iter() {
                painter.arrow(
                    point.0,
                    vec2(point.1.x - point.0.x, point.1.y - point.0.y),
                    point.2,
                );
            }
        }

        if !self.circles.is_empty(){
            for point in self.circles.clone().into_iter() {
                painter.circle(
                    point.0,
                    point.1,
                    egui::Color32::TRANSPARENT,
                    point.2
                );
            }
            
        }

        
    }
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        println!("In ui_control");

        if  self.lines.last_mut()==None{
            ui.horizontal(|ui| {
                egui::stroke_ui(ui, &mut self.lines_stroke, "Stroke");
                ui.separator();
                if ui.button("Clear Painting").clicked() {
                    self.lines.clear();
                }
            })
            
            .response
            

        }else{
            
            let res=ui.horizontal(|ui| {
                egui::stroke_ui(ui, &mut self.lines.last_mut().unwrap().1, "Stroke");
                ui.separator();
                if ui.button("Clear Painting").clicked() {
                    self.lines.clear();
                }
            })
            .response;
            if !self.lines.is_empty(){
                self.lines_stroke=self.lines.last_mut().unwrap().1;
            }
            
            res
        }
        
        
    }

    pub fn ui_content(&mut self, ui: &mut Ui, image: egui::Image, dim: Vec2) -> egui::Response {
        println!("In ui_content");

        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );

        image.paint_at(ui, response.rect);
        
        self.render_elements(painter.clone());

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
        
        let shapes = self.lines
            .iter()            
            .filter(|line| line.0.len() >= 2)
            .map(|line| {
                let points: Vec<Pos2> = line.0.iter().map(|p| to_screen * *p).collect();
                egui::Shape::line(points, line.1)
            });

        painter.extend(shapes);

        response
    }

    pub fn ui_control_arrows(&mut self, ui: &mut egui::Ui) -> egui::Response {
        println!("In ui_control arrows");
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.arrows_stroke, "Stroke");
            ui.separator();
        })
        .response
    }

    pub fn ui_content_arrows(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        dim: Vec2,
    ) -> egui::Response {
        println!("In ui_content arrows");

        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        image.paint_at(ui, response.rect);
        self.render_elements(painter.clone());
        if !self.lines.is_empty() {
            let shapes = self.lines
            .iter()            
            .filter(|line| line.0.len() >= 2)
            .map(|line| {
                let points: Vec<Pos2> = line.0.iter().map(|p| to_screen * *p).collect();
                egui::Shape::line(points, line.1)
            });


            painter.extend(shapes);
        }

        
        if ui.input(|i| i.pointer.any_down())
            && self.starting_point.x == -1.0
            && self.starting_point.y == -1.0
        {
            self.starting_point = ui.input(|i| i.pointer.interact_pos().unwrap());
        }
        if ui.input(|i| i.pointer.any_released())
            && self.final_point.x == -1.0
            && self.final_point.y == -1.0
        {
            self.final_point = ui.input(|i| i.pointer.interact_pos().unwrap());
        }
        if self.final_point.x != -1.0
            && self.final_point.y != -1.0
            && self.starting_point.x != -1.0
            && self.starting_point.y != -1.0
        {
            self.arrows.push((self.starting_point, self.final_point, self.arrows_stroke));
            self.starting_point = Pos2 { x: -1.0, y: -1.0 };
            self.final_point = Pos2 { x: -1.0, y: -1.0 };
        }

        self.render_elements(painter.clone());

        response
    }

    pub fn ui_control_circles(&mut self, ui: &mut egui::Ui) -> egui::Response {
        println!("In ui_control circles");
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.circles_stroke, "Stroke");
            ui.separator();
        })
        .response
    }

    pub fn ui_content_circles(
        &mut self,
        ui: &mut Ui,
        image: egui::Image,
        dim: Vec2,
    ) -> egui::Response {
        println!("In ui_content circles");

        let (mut response, painter) = ui.allocate_painter(dim, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        image.paint_at(ui, response.rect);
        self.render_elements(painter.clone());
        if !self.lines.is_empty() {
            let shapes = self.lines
            .iter()            
            .filter(|line| line.0.len() >= 2)
            .map(|line| {
                let points: Vec<Pos2> = line.0.iter().map(|p| to_screen * *p).collect();
                egui::Shape::line(points, line.1)
            });
            painter.extend(shapes);
        }


        if ui.input(|i| i.pointer.any_down())
            && self.circle_center.x == -1.0
            && self.circle_center.y == -1.0
        {
            self.circle_center = ui.input(|i| i.pointer.interact_pos().unwrap());
        }
        if ui.input(|i| i.pointer.any_released())
            && self.circle_center.x != -1.0
            && self.circle_center.y != -1.0
            && self.radius==-1.0
        {
            self.radius=ui.input(|i| i.pointer.interact_pos().unwrap()).x-self.circle_center.x;
            self.radius=self.radius.abs();
                     
        }

        if self.circle_center.x != -1.0
            && self.circle_center.y != -1.0
            && self.radius != -1.0
            {
                self.circles.push((self.circle_center, self.radius, self.circles_stroke));
                self.circle_center = Pos2 { x: -1.0, y: -1.0 };   
                self.radius=-1.0;  
            }

            self.render_elements(painter.clone());
        

        response
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
        dim: Vec2,
    ) -> Option<egui::Response> {
        let mut resp = None;
        self.ui_control(ui);
        ui.label("Paint with your mouse/touch!");
        ui.vertical_centered(|ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                resp = Some(self.ui_content(ui, image, dim));
            });
        });

        resp
    }

    fn ui_arrows(
        &mut self,
        ui: &mut Ui,
        image: egui::widgets::Image,
        dim: Vec2,
    ) -> Option<egui::Response> {
        let mut resp = None;
        self.ui_control_arrows(ui);
        ui.label("Paint an arrow with your mouse/touch!");
        ui.vertical_centered(|ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                resp = Some(self.ui_content_arrows(ui, image, dim));
            });
        });

        resp
    }

    fn ui_circles(
        &mut self,
        ui: &mut Ui,
        image: egui::widgets::Image,
        dim: Vec2,
    ) -> Option<egui::Response> {
        let mut resp = None;
        self.ui_control_circles(ui);
        ui.label("Paint a circle with your mouse/touch!");
        ui.vertical_centered(|ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                resp = Some(self.ui_content_circles(ui, image, dim));
            });
        });

        resp
    }
}
