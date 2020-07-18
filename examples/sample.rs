extern crate rand;
extern crate rasterizer;

use rasterizer::{entity::*, input::*, mesh::*, renderer::*};

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
    entity: Entity,
}

impl UserGame {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let mut rng = rand::thread_rng();

        let mut random_colors: Vec<Color> = Vec::with_capacity(12);

        for _ in 0..12 {
            random_colors.push(Color::RGB(rng.gen(), rng.gen(), rng.gen()));
        }

        let renderer = Renderer::new(screen_width, screen_height, "raster");
        let input: Input = Input::default();
        let mesh: Mesh = Mesh::new("C:/Dev/rasterizer/meshes/bunny.obj");
        let mut entity: Entity = Entity::new(mesh);
        entity.transform.position.z += 3.0;
        entity.update_transform();
        UserGame {
            renderer,
            input,
            delta: 0,
            theta_z: 0.0,
            entity,
        }
    }

    pub fn run(&mut self) {
        let sdl_context = self.renderer.get_sdl_context().sdl_context();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut dt: f32 = 0.0;
        'game_loop: loop {
            let before = Instant::now();

            self.input.poll_events(&mut event_pump);

            if self.input.key_pressed(Scancode::Escape) || self.input.should_quit() {
                break 'game_loop;
            }
            if self.input.key_pressed(Scancode::A) {}

            self.update(dt);
            self.render();

            self.print_fps(&before);
            dt = before.elapsed().as_millis() as f32;
        }
    }

    fn update(&mut self, dt: f32) {
        let scale = dt / 60.;
        self.theta_z += 0.3 * scale;

        self.entity.transform.rotation.y = self.theta_z;

        self.entity.transform.position.x = (self.theta_z).cos() * 2.0;
        self.entity.update_transform();
    }

    fn render(&mut self) {
        self.renderer.clear();

        self.renderer.draw_entity(&self.entity);

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
