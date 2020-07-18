use crate::defs::*;
use cgmath::*;

pub struct  Transform{
    pub position:Vec3f,
    pub scale :Vec3f,
    /*in degrees*/
    pub rotation:Vec3f,
    pub mat : Mat4,
}


impl Transform{
   pub fn new()->Self{
        
        let position = Vec3f::new(0.0,0.0,0.);
        let rotation= Vec3f::new(0.0,0.0,0.);
        let scale= Vec3f::new(1.0,1.0,1.);
        let mat =Transform::update_transform_with_vals(position, rotation, scale);
        Transform{
           position,
           scale,
           rotation, 
           mat:mat
        }
    }

    pub fn update_transform_with_vals(position:Vec3f,rotation:Vec3f,scale:Vec3f)-> Mat4{
        let rot: Matrix4<f32> = Matrix4::from_angle_x(Rad(rotation.x)) *Matrix4::from_angle_y(Rad(rotation.y))
         * Matrix4::from_angle_z(Rad(rotation.x));
    
         let translate = Matrix4::from_translation(position);
         let scale = Matrix4::from_nonuniform_scale(scale.x,scale.y,scale.z);
       
         translate*rot*scale
    }

    pub fn update_transform(&mut self){
      self.mat= Transform::update_transform_with_vals(self.position,self.rotation,self.scale);
    }
}