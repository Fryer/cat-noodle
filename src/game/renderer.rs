extern crate image;
mod vertex;
mod debug;
mod ground;
mod noodle_cat;

use std::error::Error;

use lib::rgl;

use super::state;
use ground::Ground;
use noodle_cat::NoodleCat;


pub struct Renderer {
    sprite_program: rgl::Program,
    debug_program: rgl::Program,
    debug_renderer: debug::Renderer,
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

        let mut sprite_program = Self::create_program(
            include_str!("renderer/sprite.vert"),
            include_str!("renderer/sprite.frag")
        )?;
        sprite_program.set_uniform("texture0", rgl::Uniform::Integer1(0))?;

        let debug_program = Self::create_program(
            include_str!("renderer/debug.vert"),
            include_str!("renderer/debug.frag")
        )?;

        let debug_renderer = debug::Renderer::new()?;

        let ground_sprite = Self::load_texture("img/ground.png")?;
        let ground = Ground::new()?;

        let cat_sprite = Self::load_masked_texture("img/cat_rgb.png", "img/cat_a.png")?;
        let cat = NoodleCat::new()?;

        Ok(Renderer {
            sprite_program,
            debug_program,
            debug_renderer,
            ground_sprite,
            ground,
            cat_sprite,
            cat
        })
    }


    fn create_program(vertex_source: &str, fragment_source: &str) -> Result<rgl::Program, rgl::GLError> {
        let mut vertex_shader = rgl::Shader::new(rgl::ShaderType::Vertex)?;
        vertex_shader.set_source(vertex_source)?;
        vertex_shader.compile()?;

        let mut fragment_shader = rgl::Shader::new(rgl::ShaderType::Fragment)?;
        fragment_shader.set_source(fragment_source)?;
        fragment_shader.compile()?;

        let mut program = rgl::Program::new()?;
        program.attach_shader(&vertex_shader)?;
        program.attach_shader(&fragment_shader)?;
        program.link()?;

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
        let cat = &state.cat;

        let zoom = 0.2;
        let camera = cat.path.back().unwrap();

        self.debug_renderer.update(&mut state.debug)?;

        self.ground.update(&mut state.ground)?;

        self.cat.update(&state.cat)?;

        rgl::clear(0.2, 0.15, 0.3, 1.0)?;

        self.sprite_program.use_program()?;
        Self::set_transform(&mut self.sprite_program, zoom, -camera.x, -camera.y, 1.0, 0.0)?;

        self.cat_sprite.bind(0)?;
        self.cat.render()?;

        self.ground_sprite.bind(0)?;
        self.ground.render()?;

        self.cat_sprite.bind(0)?;
        self.cat.render_near()?;

        self.debug_program.use_program()?;
        Self::set_transform(&mut self.debug_program, zoom, -camera.x, -camera.y, 1.0, 0.0)?;
        self.debug_renderer.render()?;

        Ok(())
    }


    fn set_transform(program: &mut rgl::Program, zoom: f32, x: f32, y: f32, scale: f32, angle: f32)
        -> Result<(), rgl::GLError>
    {
        let aspect = 9.0 / 16.0;
        let transform = rgl::Uniform::Matrix3x2([
            (angle.cos() * scale * aspect * zoom, -angle.sin() * scale * zoom),
            (angle.sin() * scale * aspect * zoom, angle.cos() * scale * zoom),
            (x * aspect * zoom, y * zoom)
        ]);
        program.set_uniform("transform", transform)?;
        Ok(())
    }
}
