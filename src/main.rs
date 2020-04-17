extern crate sdl2;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;
pub mod framebuffer;
pub mod sdl_helper;
pub mod renderer;
use cgmath::*;
pub fn main() -> Result<(), String> {
  
  let mut renderer = renderer::Renderer::new(600, 600, "raster");


  renderer.draw_line(vec2(0,300), vec2(300,300), Color::RGB(0, 0,255));
    renderer.present();

    let sdl_context = renderer.get_sdl_context().sdl_context();
    let mut event_pump = sdl_context.event_pump()?;

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
        // The rest of the game loop goes here...
    }

    Ok(())
}