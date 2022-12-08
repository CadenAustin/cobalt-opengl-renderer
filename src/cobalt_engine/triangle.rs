use gl::types::GLfloat;
use nalgebra as na;
pub struct Vertex {
    pub pos: [f32; 3],
}

impl Vertex {
    pub fn pack(&self) -> Vec<GLfloat> {
        let mut ret = vec![];
        for p in self.pos.iter() {
            ret.push(*p);
        }   
        ret
    }
}