use crate::defs::{Vec3f};
use crate::indexed_buffer::IndexedBuffer;
pub struct Cube{
indexed_buffer: IndexedBuffer
}


impl Cube {
   pub fn new (size:f32)->Self{
        let face = size/2.0;
        
        let mut vertices: Vec<Vec3f>=Vec::with_capacity(8);
        //  vertices.push(Vec3f::new(-face,0.0,1.0));
        //  vertices.push(Vec3f::new(0.0,face,1.0));
        //  vertices.push(Vec3f::new(face,0.0,1.0));
 
        vertices.push(Vec3f::new(-face,-face,-face));
        vertices.push(Vec3f::new(face,-face,-face));
        vertices.push(Vec3f::new(-face,face,-face));
        vertices.push(Vec3f::new(face,face,-face));
        vertices.push(Vec3f::new(-face,-face,face));
        vertices.push(Vec3f::new(face,-face,face));
        vertices.push(Vec3f::new(-face,face,face));
        vertices.push(Vec3f::new(face,face,face));



        // let indices: Vec<usize>=vec![
        //     0,1,    1,3,    3,2,    2,0,
        //     0,4,    1,5,    3,7,    2,6,    
        //     4,5,     5,7,    7,6,    6,4 
        // ];

        //  let indices: Vec<usize>=vec![
        //      0,1,2,    
        //  ];

        let indices: Vec<usize>=vec![
            0,2,1,    2,3,1,
            1,3,5,    3,7,5,
            2,6,3,    3,6,7,    
            4,5,7,    4,7,6,
            0,4,2,    2,4,6,
            0,1,4,    1,5,4
        ];

        

        Cube{
            indexed_buffer:   
            IndexedBuffer{  vertices,
            indices
        }
    }
    }

    pub fn get_indexed_buffer(&self)-> &IndexedBuffer{
      &self.indexed_buffer
    }
}

