pub mod first_window {

    use crate::{FirstWindow, ModeOptions};
    use egui::{ColorImage, ImageData, Response};
    use image::{DynamicImage, EncodableLayout, ImageBuffer};
    use rusttype::Font;
    use screenshots::Screen;

    impl FirstWindow {
        pub fn set_width_height(&mut self) {
            match self.selected_mode {
                ModeOptions::Rectangle => {
                    self.width = (self.rect_pos_f[0]) - (self.rect_pos[0]);
                    self.height = (self.rect_pos_f[1]) - self.rect_pos[1];
                }
                ModeOptions::FullScreen => {
                    self.width = self.image_texture.clone().unwrap().size[0] as f32;
                    self.height = self.image_texture.clone().unwrap().size[1] as f32;
                }
            }
            if self.current_os == "windows" {
                self.width = self.width * self.multiplication_factor.unwrap();
                self.height = self.height * self.multiplication_factor.unwrap();
                self.rect_pos[0] = self.rect_pos[0] * self.multiplication_factor.unwrap();
                self.rect_pos[1] = self.rect_pos[1] * self.multiplication_factor.unwrap();
            }
        }

        pub fn set_image_texture(&mut self) {
            for i in [0, self.screenshots_taken.len() - 1] {
                let size: [usize; 2] = [
                    self.screenshots_taken[i].width() as _,
                    self.screenshots_taken[i].height() as _,
                ];

                let mut pixels = self.screenshots_taken[i].as_flat_samples_mut();

                let immagine: egui::ColorImage =
                    egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

                self.image_texture = Some(immagine);
                self.image_buffer = Some(self.screenshots_taken[i].clone());
            }
        }

        pub fn take_screenshot(&mut self) {
            let screens = Screen::all().unwrap();
            match self.selected_mode {
                ModeOptions::Rectangle => {
                    self.set_width_height();

                    for screen in screens {
                        if self.screen_to_show.is_none() == false
                            && screen.display_info.id == self.screen_to_show.unwrap()
                        {
                            if screen.display_info.is_primary == false {
                                self.rect_pos.x -= screen.display_info.width as f32;
                            }

                            if self.rect_pos[0] < 0.0 {
                                self.rect_pos[0] += self.screen_size.unwrap()[0];
                            }
                            let mut image = screen.capture_area(
                                self.rect_pos[0] as i32,
                                self.rect_pos[1] as i32,
                                self.width as u32,
                                self.height as u32,
                            );

                            if image.is_err() == false {
                                self.screenshots_taken.push(image.unwrap());
                            } else {
                            }
                        }
                    }
                    self.set_image_texture();
                }
                ModeOptions::FullScreen => {
                    for screen in screens {
                        if screen.display_info.id == self.screen_to_show.unwrap() {
                            let image = screen.capture();
                            if image.is_err() == false {
                                self.screenshots_taken.push(image.unwrap());
                            } else {
                            }
                        }
                    }
                    self.set_image_texture();
                    self.set_width_height();
                }
            }
        }

        pub fn define_rectangle(&mut self) {
            let diff_x = self.mouse_pos_f.unwrap()[0] - self.mouse_pos.unwrap()[0];
            let diff_y = self.mouse_pos_f.unwrap()[1] - self.mouse_pos.unwrap()[1];

            if diff_x > 0.0 && diff_y > 0.0 {
                self.rect_pos = self.mouse_pos.unwrap();
                self.rect_pos_f = self.mouse_pos_f.unwrap();
            } else if diff_x < 0.0 && diff_y < 0.0 {
                self.rect_pos = self.mouse_pos_f.unwrap();
                self.rect_pos_f = self.mouse_pos.unwrap();
            } else if diff_x < 0.0 && diff_y > 0.0 {
                self.rect_pos[0] = self.mouse_pos_f.unwrap()[0];
                self.rect_pos[1] = self.mouse_pos.unwrap()[1];
                self.rect_pos_f[0] = self.mouse_pos.unwrap()[0];
                self.rect_pos_f[1] = self.mouse_pos_f.unwrap()[1];
            } else if diff_x > 0.0 && diff_y < 0.0 {
                self.rect_pos[0] = self.mouse_pos.unwrap()[0];
                self.rect_pos[1] = self.mouse_pos_f.unwrap()[1];
                self.rect_pos_f[0] = self.mouse_pos_f.unwrap()[0];
                self.rect_pos_f[1] = self.mouse_pos.unwrap()[1];
            }
        }

        pub fn load_image(&mut self, ui: &mut egui::Ui) {
            let img = ui.ctx().load_texture(
                "image_texture",
                ImageData::from(self.image_texture.clone().unwrap()),
                Default::default(),
            );
            self.image = Some(img);
        }

        pub fn edit_image(&mut self, ui: &mut egui::Ui) {
            if self.circle_pixels.is_empty() == false {
                for c in self.circle_pixels.clone() {
                    imageproc::drawing::draw_hollow_circle_mut(
                        self.image_buffer.as_mut().unwrap(),
                        (c.0.x as i32, c.0.y as i32),
                        c.1 as i32,
                        image::Rgba([c.2.r(), c.2.g(), c.2.b(), c.2.a()]),
                    );
                }
            }

            if self.square_pixels.is_empty() == false {
                let mut i = 0;

                for p in self.square_pixels.clone() {
                    i += 1;
                    let w = p.0.width() as u32;
                    let h = p.0.height() as u32;
                    let rett =
                        imageproc::rect::Rect::at(p.0.left_top().x as i32, p.0.left_top().y as i32)
                            .of_size(w, h);
                    imageproc::drawing::draw_hollow_rect_mut(
                        self.image_buffer.as_mut().unwrap(),
                        rett,
                        image::Rgba([p.1.r(), p.1.g(), p.1.b(), p.1.a()]),
                    );
                }
            }

            if self.arrow_pixels.is_empty() == false {
                for p in self.arrow_pixels.clone() {
                    let head = p.0[1];
                    for pi in p.0 {
                        imageproc::drawing::draw_line_segment_mut(
                            self.image_buffer.as_mut().unwrap(),
                            (pi.x, pi.y),
                            (head.x, head.y),
                            image::Rgba([p.1.r(), p.1.g(), p.1.b(), p.1.a()]),
                        );
                    }
                }
            }

            if self.text_pixels.is_empty() == false {
                let font_data: &[u8] = include_bytes!("../DejaVuSansMono.ttf");
                let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
                for t in self.text_pixels.clone() {
                    imageproc::drawing::draw_text_mut(
                        self.image_buffer.as_mut().unwrap(),
                        image::Rgba([t.1.r(), t.1.g(), t.1.b(), t.1.a()]),
                        t.0.x as i32,
                        t.0.y as i32,
                        rusttype::Scale {
                            x: 20.0 * self.mult_factor.unwrap().0,
                            y: 20.0 * self.mult_factor.unwrap().1,
                        },
                        &font,
                        &t.2,
                    );
                }
            }

            if self.line_pixels.is_empty() == false {
                for p in self.line_pixels.clone() {
                    if p.0.is_empty() == false {
                        for j in 0..p.0.len() - 1 {
                            let start = p.0[j];
                            let end = p.0[j + 1];

                            imageproc::drawing::draw_line_segment_mut(
                                self.image_buffer.as_mut().unwrap(),
                                (start.x, start.y),
                                (end.x, end.y),
                                image::Rgba([p.1.r(), p.1.g(), p.1.b(), p.1.a()]),
                            );
                        }
                    }
                }
            }

            let ci = ColorImage::from_rgba_unmultiplied(
                [
                    self.image_buffer.clone().unwrap().dimensions().0 as usize,
                    self.image_buffer.clone().unwrap().dimensions().1 as usize,
                ],
                self.image_buffer.clone().unwrap().as_bytes(),
            );
            let new_img =
                ui.ctx()
                    .load_texture("new image", ImageData::from(ci.clone()), Default::default());
            self.image = Some(new_img);
            self.save = true;
        }
        pub fn load_cutted_img(&mut self, ui: &mut egui::Ui, response: Option<Response>) {
            if self.current_os == "windows" {
                self.multiplication_factor = Some(1.0);
            }
            let di = DynamicImage::ImageRgba8(self.image_buffer.clone().unwrap());
            let w = f32::abs(self.to_cut_rect.unwrap().0.x - self.to_cut_rect.unwrap().1.x);
            let h = f32::abs(self.to_cut_rect.unwrap().0.y - self.to_cut_rect.unwrap().1.y);
            let cutted = di.crop_imm(
                (((self.to_cut_rect.unwrap().0.x - response.clone().unwrap().rect.left_top().x)
                    / self.shrink_fact.unwrap())
                    * self.multiplication_factor.unwrap()) as u32,
                (((self.to_cut_rect.unwrap().0.y - response.clone().unwrap().rect.left_top().y)
                    / self.shrink_fact.unwrap())
                    * self.multiplication_factor.unwrap()) as u32,
                ((w / self.shrink_fact.unwrap()) * self.multiplication_factor.unwrap()) as u32,
                ((h / self.shrink_fact.unwrap()) * self.multiplication_factor.unwrap()) as u32,
            );
            let image_buffer_cutted = Some(ImageBuffer::from(cutted.clone().into_rgba8()));
            let im_b = cutted.to_rgba8();
            let ci = ColorImage::from_rgba_unmultiplied(
                [im_b.dimensions().0 as usize, im_b.dimensions().1 as usize],
                im_b.as_bytes(),
            );
            let new_img =
                ui.ctx()
                    .load_texture("new image", ImageData::from(ci.clone()), Default::default());
            self.image = Some(new_img);
            self.width = self.image.clone().unwrap().size()[0] as f32;
            self.height = self.image.clone().unwrap().size()[1] as f32;
            self.image_buffer = image_buffer_cutted.clone();
        }
    }
}
