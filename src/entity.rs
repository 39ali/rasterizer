use crate::mesh::*;
use crate::transform::{Transform};
pub struct Entity {
    pub transform:Transform,
    pub mesh:Mesh
}



impl Entity{

    pub fn new(mesh:Mesh)->Self{
       
        let transfrom = Transform::new();
        Entity{
        transform:transfrom,
        mesh
        }
    }
   pub fn set_mesh(&mut self,mesh: Mesh){
    self.mesh= mesh;
   } 

   pub fn update_transform(&mut self ){
        self.transform.update_transform();
    }

}

