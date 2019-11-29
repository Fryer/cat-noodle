use std::mem;

use rgl;


#[repr(C)]
#[derive(Clone, Copy)]
struct Position(f32, f32);
#[repr(C)]
#[derive(Clone, Copy)]
struct TexCoord(f32, f32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex(Position, TexCoord);


pub fn create_array(vertices: &[Vertex], usage: rgl::BufferUsage) -> Result<rgl::VertexArray, rgl::GLError> {
    let mut buffer = rgl::VertexBuffer::new()?;
    buffer.set_data(&vertices, usage)?;

    let mut array = rgl::VertexArray::new(buffer)?;
    array.set_attribute_ptr(0, 2, rgl::AttributeType::Float, false, Vertex::stride(), Vertex::position_offset())?;
    array.set_attribute_ptr(1, 2, rgl::AttributeType::Float, false, Vertex::stride(), Vertex::tex_coord_offset())?;
    array.set_attribute(2, rgl::Attribute::NUByte4(255, 255, 255, 255))?;

    Ok(array)
}


impl Vertex {
    pub fn new(x: f32, y: f32, s: f32, t: f32) -> Vertex {
        Vertex(Position(x, y), TexCoord(s, t))
    }


    pub fn stride() -> usize {
        mem::size_of::<Self>()
    }


    pub fn position_offset() -> usize {
        let vertex = Vertex::new(0.0, 0.0, 0.0, 0.0);
        &vertex.0 as *const _ as usize - &vertex as *const _ as usize
    }


    pub fn tex_coord_offset() -> usize {
        let vertex = Vertex::new(0.0, 0.0, 0.0, 0.0);
        &vertex.1 as *const _ as usize - &vertex as *const _ as usize
    }
}
