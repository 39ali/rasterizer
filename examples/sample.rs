extern crate rand;
extern crate rasterizer;

use rasterizer::{cube::*, defs::*, input::*, renderer::*, transformers::*,mesh::* };

use cgmath::*;
use rand::Rng;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::time::Instant;

struct UserGame {
    renderer: Renderer,
    input: Input,

    delta: u128,
    theta_z: f32,
    random_colors: Vec<Color>,
    cube: Cube,
    mesh:Mesh
}

impl UserGame {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let mut rng = rand::thread_rng();
 
        let mut random_colors: Vec<Color> = Vec::with_capacity(12);

        for _ in 0..12 {
            random_colors.push(Color::RGB(rng.gen(), rng.gen(), rng.gen()));
        }

        let renderer = Renderer::new(screen_width, screen_height, "raster");
        let input = Input::new();
        let cube: Cube = Cube::new(0.5);
        let mesh :Mesh= Mesh::new("C:/Dev/rasterizer/meshes/bunny.obj");
        UserGame {
            renderer,
            input,
            delta: 0,
            theta_z: 0.0,
            random_colors,
            cube,
            mesh
        }
    }

    pub fn run(&mut self) {
        let sdl_context = self.renderer.get_sdl_context().sdl_context();
        let mut event_pump = sdl_context.event_pump().unwrap();
      
        'game_loop: loop {
            let before = Instant::now();

            self.input.poll_events(&mut event_pump);

            if self.input.key_pressed(Scancode::Escape) || self.input.should_quit() {
                break 'game_loop;
            }
            if self.input.key_pressed(Scancode::A) {}
            self.update();
            self.render();

           
            self.print_fps(&before);
        
        }
    }

    fn update(&self) {}

    fn render(&mut self) {
        self.renderer.clear();
        let cube_buffer = self.cube.get_indexed_buffer();
        
        let mut transformed_vertices: Vec<Vec3f> = Vec::with_capacity(self.mesh.vertices.len());
        self.theta_z = wrap_angle(self.theta_z + std::f32::consts::PI / 60.0);
        //  Matrix4::from(ortho(0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
        let rot: Matrix4<f32> = Matrix4::from_angle_y(Rad(self.theta_z));
          //  * Matrix4::from_angle_z(Rad(self.theta_z));
        for v in self.mesh.vertices.iter() {
            let mut transformed_v = *v;
            transformed_v = rot.transform_vector(transformed_v);
            transformed_v.z += 3.0;
            let sp_v = ndc_to_screen_space(
                &transformed_v,
                self.renderer.get_size().x as u32,
                self.renderer.get_size().y as u32,
            );
            transformed_vertices.push(sp_v);
        }
//println!("ypp {}",transformed_vertices.len());
        let mut indecies_iter = self.mesh.indices.iter();
    //   println!("{:?}",indecies_iter);
        let mut i = 0;
       
            while let Some(index1)=indecies_iter.next() {
              
                  
                    let index2 = indecies_iter.next();
                    let index3 = indecies_iter.next();

                    if !index2.is_some() || !index3.is_some(){
                        break
                    }
                
                    let index2 = index2.unwrap();
                    let index3=index3.unwrap();
                     // println!("{:?},{:?},{:?}", index1,index2,index3);

                    let v0 = transformed_vertices[*index1 as usize] ;
                    let v1 = transformed_vertices[*index2 as usize];
                    let v2 = transformed_vertices[*index3 as usize];
                   // println!("{:?},{:?}, {:?}",v0,v1,v2);
                    self.renderer
                        .draw_trangle(&v0.xy(), &v1.xy(), &v2.xy(), self.random_colors[i%self.random_colors.len()]);
                    i += 1;
                  //  println!("end");
                    //renderer.draw_line(&start.xy(), &end.xy(), Color::RGB(255, 255, 255));
                }
               
            
      

        self.renderer.present();
    }

    fn print_fps(&mut self, before: &Instant) {
        let after = before.elapsed();
        self.delta += after.as_millis();
        if self.delta >= 1000 {
            self.delta = 0;
            println!(
                "Ticks:{:.2?} / fps:{:?}",
                after,
                1000.0 as u128 / after.as_millis()
            );
        }
    }
}

pub fn main() {
    let mut g = UserGame::new(600, 600);
    g.run();
}

pub fn rot<R: Rotation3<f32>>(deg: f32) -> R {
    let axis = Vector3::new(0.0, 0.0, 1.0).normalize();
    Rotation3::from_axis_angle(axis, Deg(deg))
}
