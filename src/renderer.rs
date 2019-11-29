use std::{error::Error, time};

extern crate image;

use rgl;

mod vertex;
use vertex::Vertex;

mod noodle_cat;
use noodle_cat::NoodleCat;


pub struct Renderer {
    start_time: time::Instant,
    program: rgl::Program,
    square: rgl::VertexArray,
    texture: rgl::Texture,
    cat_sprite: rgl::Texture,
    cat: NoodleCat
}


impl Renderer {
    pub fn new() -> Result<Renderer, Box<dyn Error>> {
        rgl::set_blend_function(Some(rgl::BlendFunction(
            rgl::BlendFactor::SourceAlpha,
            rgl::BlendFactor::OneMinusSourceAlpha
        )))?;

        let program = Self::create_program()?;
        let square = Self::create_square()?;
        let texture = Self::load_texture("img/ball-cat.png")?;
        let cat_sprite = Self::load_texture("img/cat.png")?;
        let cat = NoodleCat::new()?;
        Ok(Renderer {
            start_time: time::Instant::now(),
            program,
            square,
            texture,
            cat_sprite,
            cat
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
        let vertices = [
            Vertex::new(-0.5, 0.5, 0.0, 0.0), Vertex::new(-0.5, -0.5, 0.0, 1.0), Vertex::new(0.5, -0.5, 1.0, 1.0),
            Vertex::new(-0.5, 0.5, 0.0, 0.0), Vertex::new(0.5, -0.5, 1.0, 1.0), Vertex::new(0.5, 0.5, 1.0, 0.0)
        ];
        let square = vertex::create_array(&vertices, rgl::BufferUsage::StaticDraw)?;
        Ok(square)
    }


    fn load_texture(file: &str) -> Result<rgl::Texture, Box<dyn Error>> {
        let image = image::open(file)?.to_rgba();
        let width = image.width();
        let height = image.height();
        let data = image.into_raw();

        let mut texture = rgl::Texture::new()?;
        texture.set_data(data.as_slice(), width as _, height as _)?;

        Ok(texture)
    }


    pub fn render(&mut self) -> Result<(), rgl::GLError> {
        let time = self.start_time.elapsed().as_secs_f64();
        let zoom = 0.5;

        let l = time.sin() as f32 * 0.1 + 0.2;
        rgl::clear(l, l, l, 1.0)?;

        self.program.use_program()?;

        self.texture.bind(0)?;
        self.set_transform(
            zoom,
            (time * 0.5).cos() as f32, (time * 0.5).sin() as f32,
            1.0, -time.rem_euclid(std::f64::consts::PI * 2.0) as f32 * 2.0
        )?;
        self.square.bind()?;
        rgl::draw(6)?;

        self.cat_sprite.bind(0)?;
        self.set_transform(zoom, 0.0, 0.0, 1.0, 0.0)?;
        self.cat.render()?;

        Ok(())
    }


    fn set_transform(&mut self, zoom: f32, x: f32, y: f32, scale: f32, angle: f32) -> Result<(), rgl::GLError> {
        let aspect = 9.0 / 16.0;
        let transform = rgl::Uniform::Matrix3x2([
            (angle.cos() * scale * aspect * zoom, -angle.sin() * scale * zoom),
            (angle.sin() * scale * aspect * zoom, angle.cos() * scale * zoom),
            (x * aspect * zoom, y * zoom)
        ]);
        self.program.set_uniform("transform", transform)?;
        Ok(())
    }
}
