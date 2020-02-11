use std::error::Error;

use lib::rgl;
use lib::math::{Vec2, vec2};

use super::state;
use super::text::{self, Font, Text};
use super::vertex::DebugVertex;


pub struct Renderer {
    vertex_array: rgl::VertexArray,
    vertices: usize,
    font: Font,
    text: Text
}


impl Renderer {
    pub fn new(library: &text::Library) -> Result<Renderer, Box<dyn Error>> {
        Ok(Renderer {
            vertex_array: DebugVertex::create_array(&[])?,
            vertices: 0,
            font: library.new_font("font/Roboto-Bold.ttf", 18)?,
            text: Text::new()
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

        let mut p = vec2(10.0, -10.0);
        self.text.add_text(&self.font, "FPS: ", p);
        let fps = info.frames.len();
        let (r, g, b) = if info.skipped_steps { (255, 0, 0) } else { (0, 255, 0) };
        self.text.add_text_rgb(&self.font, format!("{}", fps).as_str(), p + vec2(40.0, 0.0), r, g, b);
        if info.paused {
            self.text.add_text_rgb(&self.font, "Paused", p + vec2(80.0, 0.0), 255, 128, 128);
        }
        p.y -= self.font.height() * 1.5;
        self.text.add_text(&self.font, "[0]: ", p);
        let (r, g, b) = if info.show_physics { (191, 255, 191) } else { (191, 128, 128) };
        self.text.add_text_rgb(&self.font, "Debug physics", p + vec2(30.0, 0.0), r, g, b);
        p.y -= self.font.height();
        if info.show_physics {
            let (shapes, joints, aabbs, transforms, contacts) = (
                info.physics_flags.contains(state::DebugPhysics::SHAPES),
                info.physics_flags.contains(state::DebugPhysics::JOINTS),
                info.physics_flags.contains(state::DebugPhysics::AABBS),
                info.physics_flags.contains(state::DebugPhysics::TRANSFORMS),
                info.physics_flags.contains(state::DebugPhysics::CONTACTS)
            );
            self.text.add_text(&self.font, "[1]: ", p + vec2(10.0, 0.0));
            let (r, g, b) = if shapes { (191, 255, 191) } else { (191, 128, 128) };
            self.text.add_text_rgb(&self.font, "Show shapes", p + vec2(40.0, 0.0), r, g, b);
            p.y -= self.font.height();
            self.text.add_text(&self.font, "[2]: ", p + vec2(10.0, 0.0));
            let (r, g, b) = if joints { (191, 255, 191) } else { (191, 128, 128) };
            self.text.add_text_rgb(&self.font, "Show joints", p + vec2(40.0, 0.0), r, g, b);
            p.y -= self.font.height();
            self.text.add_text(&self.font, "[3]: ", p + vec2(10.0, 0.0));
            let (r, g, b) = if aabbs { (191, 255, 191) } else { (191, 128, 128) };
            self.text.add_text_rgb(&self.font, "Show AABBs", p + vec2(40.0, 0.0), r, g, b);
            p.y -= self.font.height();
            self.text.add_text(&self.font, "[4]: ", p + vec2(10.0, 0.0));
            let (r, g, b) = if transforms { (191, 255, 191) } else { (191, 128, 128) };
            self.text.add_text_rgb(&self.font, "Show transforms", p + vec2(40.0, 0.0), r, g, b);
            p.y -= self.font.height();
            self.text.add_text(&self.font, "[5]: ", p + vec2(10.0, 0.0));
            let (r, g, b) = if contacts { (191, 255, 191) } else { (191, 128, 128) };
            self.text.add_text_rgb(&self.font, "Show contacts", p + vec2(40.0, 0.0), r, g, b);
            p.y -= self.font.height();
        }
        self.text.update(true)?;

        Ok(())
    }


    fn add_line(vertices: &mut Vec<DebugVertex>, x1: f32, y1: f32, x2: f32, y2: f32, r: u8, g: u8, b: u8, a: u8) {
        vertices.extend([
            DebugVertex::new(vec2(x1, y1), r, g, b, a),
            DebugVertex::new(vec2(x2, y2), r, g, b, a)
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


    pub fn render_text(&self) -> Result<(), rgl::GLError> {
        self.font.bind(0)?;
        self.text.render()?;
        Ok(())
    }
}
