use crate::defs::*;
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

    pub fn draw_line(&mut self, start: &Vector2<f32>, end: &Vector2<f32>, color: Color) {
        // y = mx+c

        let mut x1 = start.x;
        let mut x2 = end.x;
        let mut y1 = start.y;
        let mut y2 = end.y;

        let dx = x2 - x1;
        let dy = y2 - y1;

        if dx == 0.0 && dy == 0.0 {
            self.framebuffer
                .put_pixel(start.x as usize, start.y as usize, color);
        } else if dy.abs() > dx.abs() {
            if dy < 0.0 {
                std::mem::swap(&mut x1, &mut x2);
                std::mem::swap(&mut y1, &mut y2);
            }

            let m = dx / dy;
            let mut y = y1;
            let mut last_int_y = 0;
            let mut x = x1;
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
            let mut y = y1;
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

    fn orient2d(&mut self,a: &Vec2i, b: &Vec2i, c: &Vec2i) -> i32 {
      //  println!("{:?},{:?},{:?}", a,b,c);
        (b.x-a.x)*(c.y-a.y) - (b.y-a.y)*(c.x-a.x)
    }
    //https://www.cs.bgu.ac.il/~graph161/wiki.files/09c-Rasterization.pdf
    pub fn draw_trangle(&mut self, v0: &Vec2f, v1: &Vec2f, v2: &Vec2f, color: Color) {
        let v0 =Vec2i::new (v0.x as i32,v0.y as i32);
        let v1 = Vec2i::new (v1.x as i32,v1.y as i32);
        let v2 = Vec2i::new (v2.x as i32,v2.y as i32);
       
       
        /* triangle bounding box*/
        let mut min_x = v0.x.min(v1.x).min(v2.x);
        let mut min_y = v0.y.min(v1.y).min(v2.y);
        let mut max_x = v0.x.max(v1.x).max(v2.x);
        let mut max_y = v0.y.max(v1.y).max(v2.y);

      
        // clip to screen bounds
        min_x = min_x.max(0);
        min_y = min_y.max(0);
        max_x = max_x.min(self.framebuffer.width as i32 - 1);
        max_y = max_y.min(self.framebuffer.height as i32 -1);
      
        let mut  p:Vec2i = Vec2i::new(min_x, min_y) ;
        while p.y<=max_y{
            p.x = min_x;
            while p.x<=max_x {
                // calcualte the barycentric coordinates
                let w0 = self.orient2d(&v0, &v1, &p);
                let w1 = self.orient2d(&v1, &v2, &p);
                let w2 = self.orient2d(&v2, &v0, &p);
              
               
                if w0>=0 &&w1>=0&&w2>=0{
                    //println!("x:{},y:{}",p.x,p.y);
                    self.framebuffer.put_pixel(p.x as usize, p.y as usize, color);
                }

                p.x+=1;
            }
            p.y+=1;

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

    pub fn get_size (&self) -> Vec2i{
        Vec2i::new(self.framebuffer.width as i32,self.framebuffer.height as i32)
    }
}
