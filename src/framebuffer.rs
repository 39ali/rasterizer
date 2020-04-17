
use sdl2::{
    pixels::Color
};

pub struct FrameBuffer{
    pub width:usize,
    pub height:usize,
   
    pub pixels:Vec<Color>
}


impl FrameBuffer{
    pub fn new (width:usize, height:usize)-> Self{
        let pixels = vec![Color::RGB(255,255,255);width*height];
        FrameBuffer{
            pixels,
            width,
            height
        }
    }

    pub fn put_pixel(&mut self , x:usize , y:usize , val:Color){
        self.pixels[self.width*y+x]=val;
    }

    pub fn get_pixel(&self,x:usize,y:usize)-> Option< &Color>{
         self.pixels.get(self.width*y+x)
    }
}
