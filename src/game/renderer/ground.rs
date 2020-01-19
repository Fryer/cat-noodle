use lib::rgl;
use lib::math::vec2;

use super::state;
use super::vertex::Vertex;


pub struct Ground {
    vertex_array: Option<rgl::VertexArray>,
    vertices: usize
}


impl Ground {
    pub fn new() ->Ground {
        Ground {
            vertex_array: None,
            vertices: 0
        }
    }


    pub fn update(&mut self, ground: &mut state::Ground) -> Result<(), rgl::GLError> {
        if !ground.dirty.contains(state::DirtyFlags::RENDER) {
            return Ok(());
        }
        ground.dirty -= state::DirtyFlags::RENDER;

        let mut vertices: Vec<Vertex> = Vec::with_capacity(ground.boxes.len() * 6);

        for p in ground.boxes.iter().copied() {
            vertices.extend([
                Vertex::new(p + vec2(-0.5, 0.5), vec2(0.0, 0.0)),
                Vertex::new(p + vec2(-0.5, -0.5), vec2(0.0, 1.0)),
                Vertex::new(p + vec2(0.5, -0.5), vec2(1.0, 1.0)),
                Vertex::new(p + vec2(-0.5, 0.5), vec2(0.0, 0.0)),
                Vertex::new(p + vec2(0.5, -0.5), vec2(1.0, 1.0)),
                Vertex::new(p + vec2(0.5, 0.5), vec2(1.0, 0.0))
            ].into_iter());
        }

        self.vertex_array = Some(Vertex::create_array(vertices.as_slice(), rgl::BufferUsage::StaticDraw)?);
        self.vertices = vertices.len();
        Ok(())
    }


    pub fn render(&self) -> Result<(), rgl::GLError> {
        if let Some(vertex_array) = &self.vertex_array {
            vertex_array.bind()?;
            rgl::draw(rgl::DrawMode::Triangles, 0, self.vertices as _)?;
        }
        Ok(())
    }
}
