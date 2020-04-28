
use sdl2::{
    pixels::Color
};

pub struct FrameBuffer{
    pub width:usize,
    pub height:usize,
   pub clear_color:Color,
    pub pixels:Vec<Color>
}


impl FrameBuffer{
    pub fn new (width:usize, height:usize)-> Self{

        let _clear_col:Color = Color::RGB(255,255,255);
        let pixels = vec![_clear_col;width*height];
        FrameBuffer{
            pixels,
            width,
            clear_color:Color::RGB(255,255,255),
            height
           
        }
    }

    pub fn put_pixel(&mut self , x:usize , y:usize , val:Color){
        self.pixels[self.width*y+x]=val;
    }

    pub fn get_pixel(&self,x:usize,y:usize)-> Option< &Color>{
         self.pixels.get(self.width*y+x)
    }

    pub fn set_clear_color(& mut self,color :Color){
        self.clear_color=color;
    }
    pub fn clear(&mut  self){
        self.pixels=vec![self.clear_color;self.width*self.height];
    }
}
