use lib::rgl;
use lib::math::{Vec2, vec2};

use super::vertex::{self, Vertex};


pub struct Ground {
    vertex_array: Option<rgl::VertexArray>,
    vertices: usize
}


impl Ground {
    pub fn new() -> Result<Ground, rgl::GLError> {
        Ok(Ground {
            vertex_array: None,
            vertices: 0
        })
    }


    pub fn update<B>(&mut self, boxes: B) -> Result<(), rgl::GLError>
        where B: ExactSizeIterator<Item = Vec2>
    {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(boxes.len() * 6);

        for p in boxes {
            vertices.extend([
                Vertex::new(p + vec2(-0.5, 0.5), (0.0, 0.0)),
                Vertex::new(p + vec2(-0.5, -0.5), (0.0, 1.0)),
                Vertex::new(p + vec2(0.5, -0.5), (1.0, 1.0)),
                Vertex::new(p + vec2(-0.5, 0.5), (0.0, 0.0)),
                Vertex::new(p + vec2(0.5, -0.5), (1.0, 1.0)),
                Vertex::new(p + vec2(0.5, 0.5), (1.0, 0.0))
            ].into_iter());
        }

        self.vertex_array = Some(vertex::create_array(vertices.as_slice(), rgl::BufferUsage::StaticDraw)?);
        self.vertices = vertices.len();
        Ok(())
    }


    pub fn render(&self) -> Result<(), rgl::GLError> {
        if let Some(vertex_array) = &self.vertex_array {
            vertex_array.bind()?;
            rgl::draw(0, self.vertices as _)?;
        }
        Ok(())
    }
}
