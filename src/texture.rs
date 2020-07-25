extern crate image;

use image::{open, ImageBuffer, Rgba, RgbaImage};

pub struct Texture {
    rgba: RgbaImage,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        match open(path) {
            Ok(img) => {
                let rgba = img.into_rgba();
                Texture { rgba }
            }
            Err(_) => {
                assert!(false, "image was not loaded:{}", path);
                Texture {
                    rgba: ImageBuffer::new(0, 0),
                }
            }
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        self.rgba.get_pixel(x, y).0
    }

    pub fn get_pixel_uv(&self, u: f32, v: f32) -> [u8; 4] {
        let w_half = (self.rgba.width()) as f32 - 1.0;
        let h_half = (self.rgba.height()) as f32 - 1.0;

        //   let u = u.min(1.0);
        //     let v = v.min(1.0);

        let x = w_half * u;
        let y = h_half * v;
        // if u ==0.0 || u==1.0 || v==1.0 || u==1.0
        // {
        // //println!("u:{} , v:{} , x:{} , y:{}",u,v , x,y);
        // }
        self.rgba.get_pixel(x as u32, y as u32).0
    }
}
