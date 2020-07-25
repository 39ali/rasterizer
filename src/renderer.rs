use crate::defs::*;
use crate::entity::*;
use crate::framebuffer::FrameBuffer;
use crate::sdl_helper::SdlHelper;
use crate::texture::*;
use crate::transformers::*;
use cgmath::*;
use sdl2::{pixels::Color, render::Canvas, video::Window};

extern crate rand;

pub struct Renderer {
    canvas: Canvas<Window>,
    pub sdl: SdlHelper,
    framebuffer: FrameBuffer,
}

impl Renderer {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let sdl = SdlHelper::default();
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

    // returns the area of parallelogram formed by two vectors
    fn orient3d(&mut self, a: &Vec3f, b: &Vec3f, c: &Vec3f) -> f32 {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }
    // returns the area of parallelogram formed by two vectors
    fn orient3d2(&mut self, a: &Vec3f, b: &Vec3f, c: &Vec2f) -> f32 {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }
    // https://www.cs.bgu.ac.il/~graph161/wiki.files/09c-Rasterization.pdf
    // we are drawing in CW order
    pub fn draw_triangle_with_color(&mut self, v0: &Vec3f, v1: &Vec3f, v2: &Vec3f, color: &Color) {
        // pre-compute 1 over z
        let v0 = Vec3f::new(v0.x, v0.y, 1. / v0.z);
        let v1 = Vec3f::new(v1.x, v1.y, 1.0 / v1.z);
        let v2 = Vec3f::new(v2.x, v2.y, 1.0 / v2.z);

        /* triangle bounding box*/
        let mut min_x = (v0.x as i32).min(v1.x as i32).min(v2.x as i32);
        let mut min_y = (v0.y as i32).min(v1.y as i32).min(v2.y as i32);
        let mut max_x = (v0.x as i32).max(v1.x as i32).max(v2.x as i32);
        let mut max_y = (v0.y as i32).max(v1.y as i32).max(v2.y as i32);

        // clip to screen bounds
        min_x = min_x.max(0);
        min_y = min_y.max(0);
        max_x = max_x.min((self.framebuffer.width - 1) as i32);
        max_y = max_y.min((self.framebuffer.height - 1) as i32);

        //edges of the triangle
        let edge0 = v2 - v1;
        let edge1 = v0 - v2;
        let edge2 = v1 - v0;

        //   println!("v0:{:?} , v1:{:?} , v2:{:?} , min_x:{} , min_y:{} , max_x:{} , max_y:{}", v0,v1,v2,min_x,min_y,max_x,max_y);

        let mut p: Vec2f = Vec2f::new(min_x as f32, min_y as f32);

        // we add 0.5 to center it in the middle of the pixel
        p.y += 0.5;
        while p.y <= max_y as f32 + 0.5 {
            p.x = min_x as f32 + 0.5;
            while p.x <= max_x as f32 + 0.5 {
                // calcualte the barycentric coordinates
                let  w0 = self.orient3d2(&v1, &v2, &p);
                let  w1 = self.orient3d2(&v2, &v0, &p);
                let  w2 = self.orient3d2(&v0, &v1, &p);

                let mut is_hit;

                if w0 > 0.0 {
                    is_hit = true;
                } else if w0 == 0.0 && (edge0.y == 0.0 && edge0.x > 0.0 || edge0.y > 0.0) {
                    is_hit = true;
                } else {
                    is_hit = false;
                }

                if w1 > 0.0 {
                    is_hit &= true;
                } else if w1 == 0.0 && (edge1.y == 0.0 && edge1.x > 0.0 || edge1.y > 0.0) {
                    is_hit &= true;
                } else {
                    is_hit &= false;
                }

                if w2 > 0.0 {
                    is_hit &= true;
                } else if w2 == 0.0 && (edge2.y == 0.0 && edge2.x > 0.0 || edge2.y > 0.0) {
                    is_hit &= true;
                } else {
                    is_hit &= false;
                }

                if is_hit {
                    //calculate the barycentric coordinates
                    self.framebuffer
                        .put_pixel(p.x as usize, p.y as usize, *color);
                }

                p.x += 1.0;
            }
            p.y += 1.0;
        }
    }

    pub fn draw_triangle_with_texture(
        &mut self,
        v0: &Vec3f,
        v1: &Vec3f,
        v2: &Vec3f,
        tex: &Texture,
        uv0: &Vec2f,
        uv1: &Vec2f,
        uv2: &Vec2f,
    ) {
        let uv0 = Vec2f::new(uv0.x / v0.z, uv0.y / v0.z);
        let uv1 = Vec2f::new(uv1.x / v1.z, uv1.y / v1.z);
        let uv2 = Vec2f::new(uv2.x / v2.z, uv2.y / v2.z);

        // pre-compute 1 over z
        let v0 = Vec3f::new(v0.x, v0.y, 1. / v0.z);
        let v1 = Vec3f::new(v1.x, v1.y, 1.0 / v1.z);
        let v2 = Vec3f::new(v2.x, v2.y, 1.0 / v2.z);
        let w = self.orient3d(&v0, &v1, &v2);

        /* triangle bounding box*/
        let mut min_x = (v0.x as i32).min(v1.x as i32).min(v2.x as i32);
        let mut min_y = (v0.y as i32).min(v1.y as i32).min(v2.y as i32);
        let mut max_x = (v0.x as i32).max(v1.x as i32).max(v2.x as i32);
        let mut max_y = (v0.y as i32).max(v1.y as i32).max(v2.y as i32);

        // clip to screen bounds
        min_x = min_x.max(0);
        min_y = min_y.max(0);
        max_x = max_x.min((self.framebuffer.width - 1) as i32);
        max_y = max_y.min((self.framebuffer.height - 1) as i32);

        //edges of the triangle
        let edge0 = v2 - v1;
        let edge1 = v0 - v2;
        let edge2 = v1 - v0;

        let mut p: Vec2f = Vec2f::new(min_x as f32, min_y as f32);

        // we add 0.5 to center it in the middle of the pixel
        p.y += 0.5;
        while p.y <= max_y as f32 + 0.5 {
            p.x = min_x as f32 + 0.5;
            while p.x <= max_x as f32 + 0.5 {
                // calcualte the barycentric coordinates
                let mut w0 = self.orient3d2(&v1, &v2, &p);
                let mut w1 = self.orient3d2(&v2, &v0, &p);
                let mut w2 = self.orient3d2(&v0, &v1, &p);

                let mut is_hit;

                if w0 > 0.0 {
                    is_hit = true;
                } else if w0 == 0.0 && (edge0.y == 0.0 && edge0.x > 0.0 || edge0.y > 0.0) {
                    is_hit = true;
                } else {
                    is_hit = false;
                }

                if w1 > 0.0 {
                    is_hit &= true;
                } else if w1 == 0.0 && (edge1.y == 0.0 && edge1.x > 0.0 || edge1.y > 0.0) {
                    is_hit &= true;
                } else {
                    is_hit &= false;
                }

                if w2 > 0.0 {
                    is_hit &= true;
                } else if w2 == 0.0 && (edge2.y == 0.0 && edge2.x > 0.0 || edge2.y > 0.0) {
                    is_hit &= true;
                } else {
                    is_hit &= false;
                }

                if is_hit {
                    //calculate the barycentric coordinates
                    w0 /= w;
                    w1 /= w;
                    w2 /= w;

                    let mut u =
                        (w0 as f32) * uv0.x + (w1 as f32) * uv1.x + (w2 as f32) * uv2.x as f32;
                    let mut v =
                        (w0 as f32) * uv0.y + (w1 as f32) * uv1.y + (w2 as f32) * uv2.y as f32;

                    let z_ = 1.0 / (w0 * v0.z + w1 * v1.z + w2 * v2.z);

                    u *= z_;
                    v *= z_;
                    // println!("u,v", u,v)
                    let rgba_buf = tex.get_pixel_uv(u, 1.0 - v);
                    let tex_color = Color::RGBA(rgba_buf[0], rgba_buf[1], rgba_buf[2], rgba_buf[3]);

                    self.framebuffer
                        .put_pixel(p.x as usize, p.y as usize, tex_color);
                }

                p.x += 1.0;
            }
            p.y += 1.0;
        }
    }

    pub fn draw_entity_with_texture(&mut self, entity: &Entity, texture: &Texture) {
        let mut transformed_vertices: Vec<Vec3f> = Vec::with_capacity(entity.mesh.vertices.len());

        for v in entity.mesh.vertices.iter() {
            let transformed_v = entity.transform.mat * vec4(v.x, v.y, v.z, 1.0);
            let sp_v = ndc_to_screen_space(
                &transformed_v,
                self.get_size().x as u32,
                self.get_size().y as u32,
            );
            transformed_vertices.push(sp_v);
        }

        let mut indecies_iter = entity.mesh.indices.iter();

        while let Some(index1) = indecies_iter.next() {
            let index2 = indecies_iter.next();
            let index3 = indecies_iter.next();

            if index2.is_none() || index3.is_none() {
                break;
            }

            let index2 = index2.unwrap();
            let index3 = index3.unwrap();

            let v0 = transformed_vertices[*index1 as usize];
            let v1 = transformed_vertices[*index2 as usize];
            let v2 = transformed_vertices[*index3 as usize];

            let uv0 = entity.mesh.uvs[*index1 as usize];
            let uv1 = entity.mesh.uvs[*index2 as usize];
            let uv2 = entity.mesh.uvs[*index3 as usize];
            self.draw_triangle_with_texture(&v0, &v1, &v2, texture, &uv0, &uv1, &uv2);
        }
    }

    pub fn draw_entity_with_color(&mut self, entity: &Entity, colors: &Vec<Color>) {
        let mut transformed_vertices: Vec<Vec3f> = Vec::with_capacity(entity.mesh.vertices.len());

        for v in entity.mesh.vertices.iter() {
            let transformed_v = entity.transform.mat * vec4(v.x, v.y, v.z, 1.0);
            let sp_v = ndc_to_screen_space(
                &transformed_v,
                self.get_size().x as u32,
                self.get_size().y as u32,
            );
            transformed_vertices.push(sp_v);
        }

        let mut indecies_iter = entity.mesh.indices.iter();

        while let Some(index1) = indecies_iter.next() {
            let index2 = indecies_iter.next();
            let index3 = indecies_iter.next();

            if index2.is_none() || index3.is_none() {
                break;
            }

            let index2 = index2.unwrap();
            let index3 = index3.unwrap();

            let v0 = transformed_vertices[*index1 as usize];
            let v1 = transformed_vertices[*index2 as usize];
            let v2 = transformed_vertices[*index3 as usize];

           
            self.draw_triangle_with_color(
                &v0,
                &v1,
                &v2,
                &colors[(*index2 as usize) % colors.len()],
            );
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

    pub fn get_size(&self) -> Vec2i {
        Vec2i::new(
            self.framebuffer.width as i32,
            self.framebuffer.height as i32,
        )
    }
}
