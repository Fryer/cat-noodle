use std::mem;

use lib::rgl;
use lib::math::{Vec2, vec2};


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub s: f32,
    pub t: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DebugVertex {
    pub x: f32,
    pub y: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}


impl Vertex {
    pub fn new(position: Vec2, tex_coord: Vec2) -> Vertex {
        Vertex {
            x: position.x, y: position.y,
            s: tex_coord.x, t: tex_coord.y,
            r: 255, g: 255, b: 255, a: 255
        }
    }


    pub fn rgb(position: Vec2, tex_coord: Vec2, r: u8, g: u8, b: u8) -> Vertex {
        Vertex {
            x: position.x, y: position.y,
            s: tex_coord.x, t: tex_coord.y,
            r, g, b, a: 255
        }
    }


    pub fn create_array(vertices: &[Vertex], usage: rgl::BufferUsage) -> Result<rgl::VertexArray, rgl::GLError> {
        let mut buffer = rgl::VertexBuffer::new()?;
        buffer.set_data(&vertices, usage)?;

        let mut array = rgl::VertexArray::new(buffer)?;
        array.set_attribute(0, 2, rgl::AttributeType::Float, false, Self::stride(), Self::position_offset())?;
        array.set_attribute(1, 2, rgl::AttributeType::Float, false, Self::stride(), Self::tex_coord_offset())?;
        array.set_attribute(2, 4, rgl::AttributeType::UByte, true, Self::stride(), Self::color_offset())?;

        Ok(array)
    }


    pub fn stride() -> usize {
        mem::size_of::<Vertex>()
    }


    pub fn position_offset() -> usize {
        let vertex = Vertex::new(vec2(0.0, 0.0), vec2(0.0, 0.0));
        &vertex.x as *const _ as usize - &vertex as *const _ as usize
    }


    pub fn tex_coord_offset() -> usize {
        let vertex = Vertex::new(vec2(0.0, 0.0), vec2(0.0, 0.0));
        &vertex.s as *const _ as usize - &vertex as *const _ as usize
    }


    pub fn color_offset() -> usize {
        let vertex = Vertex::new(vec2(0.0, 0.0), vec2(0.0, 0.0));
        &vertex.r as *const _ as usize - &vertex as *const _ as usize
    }
}


impl DebugVertex {
    pub fn new(position: Vec2, r: u8, g: u8, b: u8, a: u8) -> DebugVertex {
        DebugVertex {
            x: position.x, y: position.y,
            r, g, b, a
        }
    }


    pub fn create_array(vertices: &[DebugVertex]) -> Result<rgl::VertexArray, rgl::GLError> {
        let mut buffer = rgl::VertexBuffer::new()?;
        buffer.set_data(&vertices, rgl::BufferUsage::StreamDraw)?;

        let mut array = rgl::VertexArray::new(buffer)?;
        array.set_attribute(0, 2, rgl::AttributeType::Float, false, Self::stride(), Self::position_offset())?;
        array.set_attribute(1, 4, rgl::AttributeType::UByte, true, Self::stride(), Self::color_offset())?;

        Ok(array)
    }


    pub fn stride() -> usize {
        mem::size_of::<DebugVertex>()
    }


    pub fn position_offset() -> usize {
        let vertex = DebugVertex::new(vec2(0.0, 0.0), 0, 0, 0, 0);
        &vertex.x as *const _ as usize - &vertex as *const _ as usize
    }


    pub fn color_offset() -> usize {
        let vertex = DebugVertex::new(vec2(0.0, 0.0), 0, 0, 0, 0);
        &vertex.r as *const _ as usize - &vertex as *const _ as usize
    }
}
