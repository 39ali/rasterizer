use crate::defs::*;
use crate::mesh::*;
pub fn create_plane() -> Mesh {
    let mut vertices: Vec<Vec3f> = Vec::with_capacity(4);
    let mut indices: Vec<u32> = Vec::with_capacity(6);
    let mut uvs = Vec::with_capacity(4);
    let normals = vec![];

    vertices.push(Vec3f::new(-1.0, 1.0, 0.0));
    vertices.push(Vec3f::new(1.0, 1.0, 0.0));
    vertices.push(Vec3f::new(1.0, -1.0, 0.0));
    vertices.push(Vec3f::new(-1.0, -1.0, 0.0));

    indices.extend([0, 1, 3, 3, 1, 2].iter());

    uvs.push(Vec2f::new(0.0, 1.0));
    uvs.push(Vec2f::new(1.0, 1.0));
    uvs.push(Vec2f::new(1.0, 0.0));
    uvs.push(Vec2f::new(0.0, 0.0));

    Mesh {
        vertices,
        indices,
        uvs,
        normals,
    }
}
