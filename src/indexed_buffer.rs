use crate::defs::Vec3f;

pub struct IndexedBuffer {
    pub vertices: Vec<Vec3f>,
    pub indices: Vec<usize>,
}

impl IndexedBuffer {
    pub fn new(vertices: Vec<Vec3f>, indices: Vec<usize>) -> Self {
        IndexedBuffer { vertices, indices }
    }
}
