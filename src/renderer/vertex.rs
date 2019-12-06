use std::mem;

use rgl;


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


pub fn create_array(vertices: &[Vertex], usage: rgl::BufferUsage) -> Result<rgl::VertexArray, rgl::GLError> {
    let mut buffer = rgl::VertexBuffer::new()?;
    buffer.set_data(&vertices, usage)?;

    let mut array = rgl::VertexArray::new(buffer)?;
    array.set_attribute(0, 2, rgl::AttributeType::Float, false, Vertex::stride(), Vertex::position_offset())?;
    array.set_attribute(1, 2, rgl::AttributeType::Float, false, Vertex::stride(), Vertex::tex_coord_offset())?;
    array.set_attribute(2, 4, rgl::AttributeType::UByte, true, Vertex::stride(), Vertex::color_offset())?;

    Ok(array)
}


impl Vertex {
    pub fn new(x: f32, y: f32, s: f32, t: f32) -> Vertex {
        Vertex { x, y, s, t, r: 255, g: 255, b: 255, a: 255 }
    }


    pub fn rgb(x: f32, y: f32, s: f32, t: f32, r: u8, g: u8, b: u8) -> Vertex {
        Vertex { x, y, s, t, r, g, b, a: 255 }
    }


    pub fn stride() -> usize {
        mem::size_of::<Self>()
    }


    pub fn position_offset() -> usize {
        let vertex = Vertex::new(0.0, 0.0, 0.0, 0.0);
        &vertex.x as *const _ as usize - &vertex as *const _ as usize
    }


    pub fn tex_coord_offset() -> usize {
        let vertex = Vertex::new(0.0, 0.0, 0.0, 0.0);
        &vertex.s as *const _ as usize - &vertex as *const _ as usize
    }


    pub fn color_offset() -> usize {
        let vertex = Vertex::new(0.0, 0.0, 0.0, 0.0);
        &vertex.r as *const _ as usize - &vertex as *const _ as usize
    }
}
