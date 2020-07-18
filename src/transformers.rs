use crate::defs::*;


pub fn ndc_to_screen_space (ndc:&Vec4f , screen_width:u32 , screen_height:u32)->Vec3f{
let  z_inv ; 
if ndc.z !=0.0 {
z_inv =1.0/ndc.z;
}   else{
    z_inv=1.0;
}
let x = (ndc.x*z_inv+1.0)* screen_width as f32 /2.0; 
let y =(-ndc.y*z_inv+1.0)*screen_height as f32/2.0;
Vec3f::new(x ,y ,ndc.z )
}