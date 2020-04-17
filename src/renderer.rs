
use sdl2::{
    render::Canvas,
    video::Window,
    pixels::Color
};
use crate::sdl_helper::SdlHelper;
use crate::framebuffer::FrameBuffer;
use cgmath::*;

pub struct Renderer{
     canvas: Canvas<Window>,
     pub sdl:SdlHelper,
     framebuffer:FrameBuffer
}


impl Renderer{

    pub fn new (width:u32 , height :u32, title:&str)-> Self{
        let sdl = SdlHelper::new();
        let mut canvas =sdl.create_canvas(width,height,title);
        let framebuffer = FrameBuffer::new(width as usize,height as usize);
        
        sdl.put_framebuffer_in_canvas(&mut canvas,&framebuffer);
        Renderer{canvas , sdl,framebuffer}
    }

    pub fn draw_line (&mut self,start:Vector2<u32> , end:Vector2<u32>,color: Color){
        // y = mx+c 
        let m =(( end.y - start.y) as f32 )/(end.x -start.x) as f32;
        let c = (start.y as f32) -m*(start.x as f32);

        for x in start.x..end.x {
            let mut y = m*(x as f32) +c;
            y = y.round();
            self.framebuffer.put_pixel(x as usize, y as usize,color )
        }
        println!("{:?}", color);
    }
    pub fn present(&mut self){
       self.sdl.put_framebuffer_in_canvas(&mut self.canvas,&self.framebuffer);
    }


    pub fn get_sdl_context(&self)->&SdlHelper{
       &self. sdl
    }
}