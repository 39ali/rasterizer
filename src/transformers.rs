use crate::defs::*;


pub fn ndc_to_screen_space (ndc:&Vec3f , screen_width:u32 , screen_height:u32)->Vec3f{
let x = (ndc.x+1.0)* screen_width as f32 /2.0; 
let y =(-ndc.y+1.0)*screen_height as f32/2.0;
Vec3f::new(x ,y ,ndc.z )
}