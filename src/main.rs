extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;
pub mod framebuffer;
pub mod renderer;
pub mod sdl_helper;
pub mod transformers;
pub mod cube;
pub mod defs;
pub mod indexed_buffer;
use std::time::Instant;
use cube::*;
use transformers::*;
use defs::*;
use cgmath::*;
pub fn main() -> Result<(), String> {
    let screen_width:u32 = 620;
    let screen_height:u32=620;
    let mut renderer = renderer::Renderer::new(screen_width, screen_height, "raster");
    
    let sdl_context = renderer.get_sdl_context().sdl_context();
    let mut event_pump = sdl_context.event_pump()?;
    let mut delta: u128 = 0;
    let mut theta_z = 0.0;
    
    let cube :Cube = Cube::new(0.5);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let before = Instant::now();
        renderer.clear();
        let cube_buffer = cube.get_index_buffer();
        let mut transformed_vertices:Vec<Vec3f>= Vec::with_capacity(cube_buffer.vertices.len());
        theta_z=wrap_angle(theta_z+ std::f32::consts::PI/60.0);
      //  Matrix4::from(ortho(0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
        let rot:Matrix4<f32>=Matrix4::from_angle_x(Rad(theta_z))
        *Matrix4::from_angle_y(Rad(theta_z))
        *Matrix4::from_angle_z(Rad(theta_z));
         
        for v in cube_buffer.vertices.iter() {
             let mut  transformed_v  =*v;
             transformed_v = rot.transform_vector(transformed_v);
                transformed_v.z+=2.0;
            let  sp_v= ndc_to_screen_space(&transformed_v, screen_width, screen_height);
            transformed_vertices.push(sp_v);
        }

        let mut indecies_iter = cube_buffer.indices.iter();
        loop{
          match indecies_iter.next(){
              Some(index1)=>{
               let index2= indecies_iter.next().unwrap();
               let start = transformed_vertices[*index1];
               let end = transformed_vertices[*index2];
               println!("{:?},{:?}",start,end);
               renderer.draw_line(start.xy(), end.xy(), Color::RGB(255, 255, 255));
            },
              None=>{break;}
          }

        }   
        
        
      
        renderer.present();

        let after = before.elapsed();
        delta += after.as_millis();
        if delta >= 1000 {
            delta = 0;
            println!(
                "Ticks:{:.2?} / fps:{:?}",
                after,
                1000.0 as u128 / after.as_millis()
            );
        }
    }

    Ok(())
}


pub fn rot<R: Rotation3<f32>>(deg:f32) -> R {
    let axis = Vector3::new(0.0, 0.0, 1.0).normalize();
    Rotation3::from_axis_angle(axis, Deg(deg))
}