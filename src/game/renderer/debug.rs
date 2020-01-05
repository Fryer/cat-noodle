use lib::rgl;
use lib::math::{Vec2, vec2};

use super::vertex::DebugVertex;

use super::state;


pub struct Renderer {
    vertex_array: rgl::VertexArray,
    vertices: usize
}


impl Renderer {
    pub fn new() -> Result<Renderer, rgl::GLError> {
        let vertex_array = DebugVertex::create_array(&[])?;

        Ok(Renderer {
            vertex_array,
            vertices: 0
        })
    }


    pub fn update(&mut self, info: &mut state::DebugInfo) -> Result<(), rgl::GLError> {
        let mut vertices: Vec<DebugVertex> = Vec::new();

        for shape in info.shapes.drain(..) {
            let state::DebugColor(r, g, b, a) = shape.1;
            match shape.0 {
                state::DebugShape::Line(x1, y1, x2, y2) => {
                    Self::add_line(&mut vertices, x1, y1, x2, y2, r, g, b, a);
                }
                state::DebugShape::Circle(x, y, radius) => {
                    Self::add_circle(&mut vertices, x, y, radius, r, g, b, a);
                }
            }
        }

        self.vertex_array.buffer.set_data(vertices.as_slice(), rgl::BufferUsage::StreamDraw)?;
        self.vertices = vertices.len();
        Ok(())
    }


    fn add_line(vertices: &mut Vec<DebugVertex>, x1: f32, y1: f32, x2: f32, y2: f32, r: u8, g: u8, b: u8, a: u8) {
        vertices.extend([
            DebugVertex::new((x1, y1), r, g, b, a),
            DebugVertex::new((x2, y2), r, g, b, a)
        ].into_iter());
    }


    fn add_circle(vertices: &mut Vec<DebugVertex>, x: f32, y: f32, radius: f32, r: u8, g: u8, b: u8, a: u8) {
        let mut p2 = vec2(radius, 0.0);
        let rot = Vec2::from_angle(std::f32::consts::PI / 16.0);
        for _ in 0..32 {
            let p1 = p2;
            p2 = p2.rotated(rot);
            Self::add_line(vertices, x + p1.x, y + p1.y, x + p2.x, y + p2.y, r, g, b, a);
        }
    }


    pub fn render(&self) -> Result<(), rgl::GLError> {
        self.vertex_array.bind()?;
        rgl::draw(rgl::DrawMode::Lines, 0, self.vertices as _)?;
        Ok(())
    }
}
