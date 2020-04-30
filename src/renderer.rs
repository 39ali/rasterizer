use crate::framebuffer::FrameBuffer;
use crate::sdl_helper::SdlHelper;
use cgmath::*;
use sdl2::{pixels::Color, render::Canvas, video::Window};

pub struct Renderer {
    canvas: Canvas<Window>,
    pub sdl: SdlHelper,
    framebuffer: FrameBuffer,
}

impl Renderer {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let sdl = SdlHelper::new();
        let mut canvas = sdl.create_canvas(width, height, title);
        let framebuffer = FrameBuffer::new(width as usize, height as usize);

        sdl.put_framebuffer_in_canvas(&mut canvas, &framebuffer);
        Renderer {
            canvas,
            sdl,
            framebuffer,
        }
    }

    pub fn draw_line(&mut self, start: Vector2<f32>, end: Vector2<f32>, color: Color) {
        // y = mx+c

        let mut x1 = start.x;
        let mut x2 = end.x;
        let mut y1 = start.y;
        let mut y2 = end.y;

        let dx = x2 -x1;
        let dy = y2 - y1;
      
        if dx == 0.0 && dy == 0.0 {
            self.framebuffer
                .put_pixel(start.x as usize, start.y as usize, color);
        }
         else if dy.abs() > dx.abs() {
            if dy < 0.0 {
                std::mem::swap(&mut x1, &mut x2);
                std::mem::swap(&mut y1, &mut y2);
            }

            let m = dx / dy;
            let mut y = y1;
            let mut last_int_y = 0;
            let mut x=x1;
            loop {
              
                if y >= y2 {
                    break;
                }
                y += 1.0;
                x += m;
                last_int_y = y.trunc() as i32;
                self.framebuffer
                    .put_pixel(x as usize, last_int_y as usize, color);
            }

            if y2.trunc() as i32 > last_int_y {
                self.framebuffer.put_pixel(x2 as usize, y2 as usize, color);
            }
        } else {
            if dx < 0.0 {
                std::mem::swap(&mut x1, &mut x2);
                std::mem::swap(&mut y1, &mut y2);
            }

            let m = dy / dx;
            let mut x = x1;
            let mut last_int_x = 0;
            let mut y=y1;
            loop {
              
                if x >= x2 {
                    break;
                }
                x += 1.0;
                y += m;
                last_int_x = x.trunc() as i32;
                self.framebuffer
                    .put_pixel(last_int_x as usize, y as usize, color);
            }

            if x2.trunc() as i32 > last_int_x {
                self.framebuffer.put_pixel(x2 as usize, y2 as usize, color);
            }
        }
    }

    //TODO:make this faster
    pub fn present(&mut self) {
        self.sdl
            .put_framebuffer_in_canvas(&mut self.canvas, &self.framebuffer);
    }

    pub fn clear(&mut self) {
        self.framebuffer.clear();
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.framebuffer.set_clear_color(color);
    }
    pub fn get_sdl_context(&self) -> &SdlHelper {
        &self.sdl
    }
}
