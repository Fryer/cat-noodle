use std::{error::Error, time};

extern crate image;

use rgl;

mod vertex;

mod noodle_cat;
use noodle_cat::NoodleCat;


pub struct Renderer {
    start_time: time::Instant,
    program: rgl::Program,
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
        let cat_sprite = Self::load_texture("img/cat.png")?;
        let cat = NoodleCat::new()?;
        Ok(Renderer {
            start_time: time::Instant::now(),
            program,
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
        let zoom = 0.2;

        let path: Vec<_> = (0..50).map(|x| (
            (time * 0.8 + x as f64 * 0.08).sin() as f32 * 6.0,
            (time * 1.6 + x as f64 * 0.16).sin() as f32 * 4.0
        )).collect();
        self.cat.update(path.as_slice())?;

        rgl::clear(0.2, 0.15, 0.3, 1.0)?;

        self.program.use_program()?;

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
