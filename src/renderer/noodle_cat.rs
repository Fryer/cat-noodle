use rgl;

use super::vertex::{self, Vertex};


pub struct NoodleCat {
    vertex_array: rgl::VertexArray
}


impl NoodleCat {
    pub fn new() -> Result<NoodleCat, rgl::GLError> {
        let vertices = [
            Vertex::new(-0.5, 0.5, 0.0, 0.0), Vertex::new(-0.5, -0.5, 0.0, 0.5), Vertex::new(0.5, -0.5, 0.5, 0.5),
            Vertex::new(-0.5, 0.5, 0.0, 0.0), Vertex::new(0.5, -0.5, 0.5, 0.5), Vertex::new(0.5, 0.5, 0.5, 0.0)
        ];
        let vertex_array = vertex::create_array(&vertices, rgl::BufferUsage::StreamDraw)?;

        Ok(NoodleCat {
            vertex_array
        })
    }


    pub fn render(&self) -> Result<(), rgl::GLError> {
        self.vertex_array.bind()?;
        rgl::draw(6)?;
        Ok(())
    }
}
