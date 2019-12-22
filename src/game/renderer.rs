use std::{error::Error, time};

extern crate image;

use lib::rgl;

mod vertex;

mod ground;
use ground::Ground;

mod noodle_cat;
use noodle_cat::NoodleCat;

use super::state;


pub struct Renderer {
    start_time: time::Instant,
    program: rgl::Program,
    ground_sprite: rgl::Texture,
    ground: Ground,
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

        let ground_sprite = Self::load_texture("img/ground.png")?;
        let ground = Ground::new()?;

        let cat_sprite = Self::load_masked_texture("img/cat_rgb.png", "img/cat_a.png")?;
        let cat = NoodleCat::new()?;

        Ok(Renderer {
            start_time: time::Instant::now(),
            program,
            ground_sprite,
            ground,
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


    fn load_masked_texture(file: &str, alpha: &str) -> Result<rgl::Texture, Box<dyn Error>> {
        let mut image = image::open(file)?.to_rgba();
        let mask = image::open(alpha)?.to_luma();
        for (x, y, color) in image.enumerate_pixels_mut() {
            if x < mask.width() && y < mask.height() {
                color.0[3] = mask.get_pixel(x, y).0[0];
            }
        }
        let width = image.width();
        let height = image.height();
        let data = image.into_raw();

        let mut texture = rgl::Texture::new()?;
        texture.set_data(data.as_slice(), width as _, height as _)?;

        Ok(texture)
    }


    pub fn render(&mut self, state: &mut state::State) -> Result<(), Box<dyn Error>> {
        let _time = self.start_time.elapsed().as_secs_f64();
        let zoom = 0.2;

        let ground = &mut state.ground;
        if ground.dirty.contains(state::DirtyFlags::RENDER) {
            self.ground.update(ground.boxes.iter().copied())?;
            ground.dirty -= state::DirtyFlags::RENDER;
        }

        let cat = &state.cat;
        self.cat.update(cat.path.iter().copied(), cat.tail.iter().copied())?;

        rgl::clear(0.2, 0.15, 0.3, 1.0)?;

        self.program.use_program()?;
        let camera = cat.path.back().unwrap();
        self.set_transform(zoom, -camera.x, -camera.y, 1.0, 0.0)?;

        self.cat_sprite.bind(0)?;
        self.cat.render()?;

        self.ground_sprite.bind(0)?;
        self.ground.render()?;

        self.cat_sprite.bind(0)?;
        self.cat.render_near()?;

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
