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
        let cat_sprite = Self::load_masked_texture("img/cat_rgb.png", "img/cat_a.png")?;
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


    #[allow(dead_code)]
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


    pub fn render(&mut self, state: &super::State) -> Result<(), Box<dyn Error>> {
        let _time = self.start_time.elapsed().as_secs_f64();
        let zoom = 0.2;

        let cat = &state.cat;
        let mindiff = std::f32::EPSILON * 1000.0;
        let last = *cat.path.back().unwrap();
        let diff = (cat.position.0 - last.0, cat.position.1 - last.1);
        let diff_len = diff.0.hypot(diff.1);
        if diff_len > mindiff {
            let pos_norm = (last.0 + diff.0 / diff_len * 0.1, last.1 + diff.1 / diff_len * 0.1);
            let path = cat.path.iter().copied().skip(1).chain(std::iter::once(pos_norm));
            let path = cat.path.iter().copied().zip(path).map(|(p, next)| (
                p.0 + (next.0 - p.0) * 10.0 * diff_len,
                p.1 + (next.1 - p.1) * 10.0 * diff_len
            )).collect::<Vec<_>>();
            let tail = cat.tail.iter().copied();
            let tail = cat.tail.iter().copied().skip(1).zip(tail).map(|(p, next)| (
                p.0 + (next.0 - p.0) * 10.0 * diff_len,
                p.1 + (next.1 - p.1) * 10.0 * diff_len
            ));
            let tail = std::iter::once(path[0]).chain(tail).collect::<Vec<_>>();
            self.cat.update(path.iter().copied(), tail.iter().copied())?;
        }
        else {
            self.cat.update(state.cat.path.iter().copied(), state.cat.tail.iter().copied())?;
        }

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
