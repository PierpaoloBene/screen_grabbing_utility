pub mod first_window {

    use egui::ImageData;
    use image::GenericImage;
    use screenshots::Screen;

    use crate::{FirstWindow, ModeOptions};
    impl FirstWindow {
        pub fn set_width_height(&mut self) {
            match self.selected_mode {
                ModeOptions::Rectangle => {
                    self.width = self.rect_pos_f[0] - self.rect_pos[0];
                    self.height = self.rect_pos_f[1] - self.rect_pos[1];
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
            println!("{}", self.screenshots_taken.len());
            for i in [0, self.screenshots_taken.len()-1] {
                let size: [usize; 2] = [
                    self.screenshots_taken[i].width() as _,
                    self.screenshots_taken[i].height() as _,
                ];
                let mut pixels = self.screenshots_taken[i].as_flat_samples_mut();
                // for mut p in pixels.as_mut_slice().into_iter(){
                //     if *p>250{
                //        *p= 10;
                        
                //     }
                   
                // }
                let immagine: egui::ColorImage =
                    egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

                self.image_texture = Some(immagine);
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
                            println!("{}",screen.display_info.scale_factor);
                            // println!("{:?} {} {} ", self.rect_pos, self.height, self.width);
                            // println!("{} {} ", screen.display_info.width, screen.display_info.height);
                            if screen.display_info.is_primary==false{
                                self.rect_pos.x-=screen.display_info.width as f32;
                            }
                            // println!("{:?} {} {} ", self.rect_pos, self.height, self.width);
                            // println!("{} {} ", screen.display_info.width, screen.display_info.height);
                            let mut image = screen.capture_area(
                                self.rect_pos[0] as i32,
                                self.rect_pos[1] as i32,
                                self.width as u32,
                                self.height as u32,
                            );
                            if image.is_err() == false {
                                let mut sub_img=image.as_mut().unwrap().sub_image(self.rect_pos[0] as u32, self.rect_pos[1] as u32, self.width as u32, self.height as u32);
                                self.screenshots_taken.push(image.unwrap());
                            }else{                                
                                println!("{:?}", image);
                            }
                        }
                    }
                    self.set_image_texture();
                }
                ModeOptions::FullScreen => {
                    //std::thread::sleep(Duration::from_secs(self.selected_timer_numeric));
                    for screen in screens {
                        if self.screen_to_show.is_none() == false
                            && self.screen_to_show.unwrap() == screen.display_info.id
                        {
                            let image = screen.capture();
                            if image.is_err() == false {
                                self.screenshots_taken.push(image.unwrap());
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
                println!("sono in basso a destra");
            } else if diff_x < 0.0 && diff_y < 0.0 {
                println!("sono in alto a sinistra");
                self.rect_pos = self.mouse_pos_f.unwrap();
                self.rect_pos_f = self.mouse_pos.unwrap();
            } else if diff_x < 0.0 && diff_y > 0.0 {
                println!("sono in basso a sinistra");
                self.rect_pos[0] = self.mouse_pos_f.unwrap()[0];
                self.rect_pos[1] = self.mouse_pos.unwrap()[1];
                self.rect_pos_f[0] = self.mouse_pos.unwrap()[0];
                self.rect_pos_f[1] = self.mouse_pos_f.unwrap()[1];
            } else if diff_x > 0.0 && diff_y < 0.0 {
                println!("sono in alto a destra");
                self.rect_pos[0] = self.mouse_pos.unwrap()[0];
                self.rect_pos[1] = self.mouse_pos_f.unwrap()[1];
                self.rect_pos_f[0] = self.mouse_pos_f.unwrap()[0];
                self.rect_pos_f[1] = self.mouse_pos.unwrap()[1];
            }
        }

        pub fn load_image(&mut self, ui: &mut egui::Ui) {
            
            let img = ui.ctx().load_texture(
                "ao",
                ImageData::from(self.image_texture.clone().unwrap()),
                Default::default(),
            );
            self.image = Some(img);
        }
    }
}
