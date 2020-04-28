extern crate sdl2;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;
pub mod framebuffer;
pub mod sdl_helper;
pub mod renderer;
use cgmath::*;
use std::time::Instant;

pub fn main() -> Result<(), String> {
  
  let mut renderer = renderer::Renderer::new(600, 600, "raster");


  
    let sdl_context = renderer.get_sdl_context().sdl_context();
    let mut event_pump = sdl_context.event_pump()?;
    let mut a = 0.0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let before = Instant::now();
        a+= 1.0/5.0;
        renderer.clear();
        renderer.draw_line(vec2(a as u32,300), vec2(300,300), Color::RGB(0, 0,255));
        renderer.present();       
        let after=before.elapsed();
    
        println!("Tick:{:.2?} / fps:{:?}",after,1000.0 as u128/after.as_millis() );
    }

    Ok(())
}