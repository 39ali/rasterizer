use crate::defs::Vec3f;
use std::time::{Instant};
extern crate tobj;
pub struct Mesh {
    pub vertices: Vec<Vec3f>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(file_name: &str) -> Self {

        let now = Instant::now();

        let err_msg = &("failed to load obj file".to_owned() + &file_name.to_owned());
        let (modeles, materials) = tobj::load_obj(file_name, false).expect(err_msg);
        println!("laoding {:} ...",file_name);
        println!("# of models:{}", modeles.len());
        println!("# of mats:{}", materials.len());
        for (i, m) in modeles.iter().take(1).enumerate() {
            let mesh = &m.mesh;
            println!("model[{}].name= {}", i, m.name);
            println!("model[{}].material_id={:?}", i, mesh.material_id);

            println!("model[{}].vertices #={}", i, mesh.positions.len()/3);
            assert!(mesh.positions.len() % 3 == 0);

            println!("model[{}].faces #={}", i, mesh.num_face_indices.len());
            println!("model[{}].indices #={}", i, mesh.num_face_indices.len()*3);
           
            let mut vertices: Vec<Vec3f> = Vec::with_capacity(mesh.positions.len());
            let mut indices =  Vec::with_capacity(mesh.num_face_indices.len()*3);
            

            let mut next_face = 0;
            for f in 0..mesh.num_face_indices.len() {
                let end = next_face + mesh.num_face_indices[f] as usize;
                
                let face_indices= &mesh.indices[next_face..end];
                indices.extend_from_slice(face_indices);
               // println!("    face[{}] = {:?}", f, face_indices);
                next_face = end;
            }

         

            for vi in 0..mesh.positions.len() / 3 {
                vertices.push(Vec3f::new(
                    mesh.positions[3 * vi + 0],
                    mesh.positions[3 * vi + 1],
                    mesh.positions[3 * vi + 2],
                ));
            }

            println!("loading mesh took:{} sec", now.elapsed().as_secs());
            return Mesh {
                indices: indices.to_owned(),
                vertices: vertices,
            };
        }

        Mesh {
            indices: vec![],
            vertices: vec![],
        }
    }
}
