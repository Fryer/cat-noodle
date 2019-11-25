use std::{
    error::Error,
    mem,
    time
};

extern crate image;

use rgl;


#[repr(C)]
struct Position(f32, f32);
#[repr(C)]
struct TexCoord(f32, f32);

#[repr(C)]
struct Vertex(Position, TexCoord);

pub struct Renderer {
    start_time: time::Instant,
    program: rgl::Program,
    square: rgl::VertexArray,
    texture: rgl::Texture
}


impl Vertex {
    pub fn new(x: f32, y: f32, s: f32, t: f32) -> Vertex {
        Vertex(Position(x, y), TexCoord(s, t))
    }
}


impl Renderer {
    pub fn new() -> Result<Renderer, Box<dyn Error>> {
        let program = Self::create_program()?;
        let square = Self::create_square()?;
        let texture = Self::create_texture()?;
        Ok(Renderer {
            start_time: time::Instant::now(),
            program,
            square,
            texture
        })
    }


    fn create_program() -> Result<rgl::Program, rgl::GLError> {
        let mut vertex_shader = rgl::Shader::new(rgl::ShaderType::Vertex)?;
        let source = include_str!("sprite.vert");
        vertex_shader.set_source(source)?;
        vertex_shader.compile()?;

        let mut fragment_shader = rgl::Shader::new(rgl::ShaderType::Fragment)?;
        let source = include_str!("sprite.frag");
        fragment_shader.set_source(source)?;
        fragment_shader.compile()?;

        let mut program = rgl::Program::new()?;
        program.attach_shader(&vertex_shader)?;
        program.attach_shader(&fragment_shader)?;
        program.link()?;

        program.set_uniform("texture0", rgl::Uniform::Integer1(0))?;

        Ok(program)
    }


    fn create_square() -> Result<rgl::VertexArray, rgl::GLError> {
        let vertex = Vertex::new(0.0, 0.0, 0.0, 0.0);
        let stride = mem::size_of_val(&vertex);
        let position_offset = &vertex.0 as *const _ as usize - &vertex as *const _ as usize;
        let texcoord_offset = &vertex.1 as *const _ as usize - &vertex as *const _ as usize;

        let mut buffer = rgl::VertexBuffer::new()?;
        let w = 0.5;
        let h = 0.5;
        let vertices = [
            Vertex::new(-w, h, 0.0, 0.0), Vertex::new(-w, -h, 0.0, 1.0), Vertex::new(w, -h, 1.0, 1.0),
            Vertex::new(-w, h, 0.0, 0.0), Vertex::new(w, -h, 1.0, 1.0), Vertex::new(w, h, 1.0, 0.0)
        ];
        buffer.set_data(&vertices, rgl::BufferUsage::StaticDraw)?;

        let mut sprite = rgl::VertexArray::new(buffer)?;
        sprite.set_attribute_ptr(0, 2, rgl::AttributeType::Float, false, stride, position_offset)?;
        sprite.set_attribute_ptr(1, 2, rgl::AttributeType::Float, false, stride, texcoord_offset)?;
        sprite.set_attribute(2, rgl::Attribute::NUByte4(255, 255, 255, 255))?;

        Ok(sprite)
    }


    fn create_texture() -> Result<rgl::Texture, Box<dyn Error>> {
        let cat_image = image::open("img/cat.png")?.to_rgba();
        let cat_width = cat_image.width();
        let cat_height = cat_image.height();
        let cat_data = cat_image.into_raw();

        let mut texture = rgl::Texture::new()?;
        texture.set_data(cat_data.as_slice(), cat_width as _, cat_height as _)?;

        Ok(texture)
    }


    pub fn render(&mut self) -> Result<(), rgl::GLError> {
        let time = time::Instant::now().duration_since(self.start_time).as_secs_f64();

        let l = time.sin() as f32 * 0.1 + 0.2;
        rgl::clear(l, l, l, 1.)?;

        self.program.use_program()?;

        let aspect = 9.0 / 16.0;
        let transform = rgl::Uniform::Matrix3x2([
            (time.cos() as f32 * aspect, -time.sin() as f32),
            (time.sin() as f32 * aspect, time.cos() as f32),
            ((time * 0.5).cos() as f32 * 0.5 * aspect, (time * 0.5).sin() as f32 * 0.5)
        ]);

        self.texture.bind(0)?;
        self.program.set_uniform("transform", transform)?;
        self.square.bind()?;
        rgl::draw(6)?;

        Ok(())
    }
}
