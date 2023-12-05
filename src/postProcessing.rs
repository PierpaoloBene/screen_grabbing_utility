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
    lines: Vec<Vec<Pos2>>,
    lines_stroke: Stroke,

    starting_point: Pos2,
    final_point: Pos2,
    arrows: Vec<(Pos2, Pos2)>,
    arrows_stroke:Stroke,
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
                    self.arrows_stroke,
                );
            }
        }

        
    }
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        println!("In ui_control");
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.lines_stroke, "Stroke");
            ui.separator();
            if ui.button("Clear Painting").clicked() {
                self.lines.clear();
            }
        })
        .response
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
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
            response.mark_changed();
        }

        let shapes = self
            .lines
            .iter()
            .filter(|line| line.len() >= 2)
            .map(|line| {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                egui::Shape::line(points, self.lines_stroke)
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

        if !self.lines.is_empty() {
            let shapes = self
                .lines
                .iter()
                .filter(|line| line.len() >= 2)
                .map(|line| {
                    let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                    egui::Shape::line(points, self.lines_stroke)
                });

            painter.extend(shapes);
        }

        // self.stroke = Stroke::new(3.0, egui::Color32::WHITE);
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
            self.arrows.push((self.starting_point, self.final_point));
            self.starting_point = Pos2 { x: -1.0, y: -1.0 };
            self.final_point = Pos2 { x: -1.0, y: -1.0 };
        }

        for point in self.arrows.clone().into_iter() {
            painter.arrow(
                point.0,
                vec2(point.1.x - point.0.x, point.1.y - point.0.y),
                self.arrows_stroke,
            );
        }

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
}
