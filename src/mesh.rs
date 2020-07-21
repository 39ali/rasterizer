use crate::defs::*;
use std::time::Instant;
extern crate tobj;
pub struct Mesh {
    pub vertices: Vec<Vec3f>,
    pub indices: Vec<u32>,
    pub uvs: Vec<Vec2f>,
    pub normals: Vec<Vec3f>,
}

impl Mesh {
    pub fn new(file_name: &str) -> Self {
        let now = Instant::now();
        // TODO: read more meshes from the model
        let err_msg = &("failed to load obj file".to_owned() + &file_name.to_owned());
        let (modeles, materials) = tobj::load_obj(file_name, false).expect(err_msg);
        println!("laoding {:} ...", file_name);
        println!("# of models:{}", modeles.len());
        println!("# of mats:{}", materials.len());
        for (i, m) in modeles.iter().take(1).enumerate() {
            let mesh = &m.mesh;
            println!("model[{}].name= {}", i, m.name);
            println!("model[{}].material_id={:?}", i, mesh.material_id);

            println!("model[{}].vertices #={}", i, mesh.positions.len() / 3);
            assert!(mesh.positions.len() % 3 == 0);

            println!("model[{}].faces #={}", i, mesh.num_face_indices.len());
            println!("model[{}].indices #={}", i, mesh.num_face_indices.len() * 3);
            
            let mut vertices: Vec<Vec3f> = Vec::with_capacity(mesh.positions.len());
            let mut indices = Vec::with_capacity(mesh.num_face_indices.len() * 3);

            let mut next_face = 0;
            //load indices
            for f in 0..mesh.num_face_indices.len() {
                let end = next_face + mesh.num_face_indices[f] as usize;

                let face_indices = &mesh.indices[next_face..end];
                indices.extend_from_slice(face_indices);
                // println!("    face[{}] = {:?}", f, face_indices);
                next_face = end;
            }

            //load vertices
            for vi in 0..mesh.positions.len() / 3 {
                vertices.push(Vec3f::new(
                    mesh.positions[3 * vi],
                    mesh.positions[3 * vi + 1],
                    mesh.positions[3 * vi + 2],
                ));
            }

            // load uv
            let mut uvs:Vec<Vec2f> = Vec::with_capacity(mesh.texcoords.len()/2);
             for vi in 0..mesh.texcoords.len() / 2 {
                uvs.push(Vec2f::new(
                    mesh.texcoords[2 * vi],
                    mesh.texcoords[2 * vi + 1],
                ));
            }

            //load normals
            let mut normals:Vec<Vec3f> = Vec::with_capacity(mesh.normals.len()/3);
             for vi in 0..mesh.normals.len() / 3 {
                normals.push(Vec3f::new(
                    mesh.normals[3 * vi],
                    mesh.normals[3 * vi + 1],
                    mesh.normals[3 * vi + 2]
                ));
            }


            assert!(normals.len()==vertices.len());
            assert!(uvs.len()==vertices.len());

            println!("loading mesh took:{} sec", now.elapsed().as_secs());
            return Mesh { indices, vertices ,uvs,normals};
        }

        Mesh {
            indices: vec![],
            vertices: vec![],
            uvs:vec![],
            normals:vec![]
        }
    }
}
