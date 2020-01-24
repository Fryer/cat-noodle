extern crate image;
mod vertex;
mod text;
mod debug;
mod ground;
mod noodle_cat;

use std::error::Error;

use lib::rgl;
use lib::math::vec2;

use super::state;
use text::{Font, Text};
use ground::Ground;
use noodle_cat::NoodleCat;


pub struct Renderer {
    sprite_program: rgl::Program,
    debug_program: rgl::Program,
    debug_renderer: debug::Renderer,
    ground_sprite: rgl::Texture,
    ground: Ground,
    cat_sprite: rgl::Texture,
    cat: NoodleCat,
    font: Font,
    text: Text
}


impl Renderer {
    pub fn new() -> Result<Renderer, Box<dyn Error>> {
        rgl::set_blend_function(Some(rgl::BlendFunction(
            rgl::BlendFactor::One,
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
        let ground = Ground::new();

        let cat_sprite = Self::load_texture("img/cat.png")?;
        let cat = NoodleCat::new()?;

        let font = text::Library::new()?.new_font("font/Roboto-Regular.ttf", 24)?;
        let mut text = Text::new();
        text.add_text(&font, "Cat Noodle!", vec2(10.0, -10.0));
        text.add_text(&font, "~~ =^-^= ~~", vec2(10.0, -10.0 - font.height()));
        text.update(false)?;

        Ok(Renderer {
            sprite_program,
            debug_program,
            debug_renderer,
            ground_sprite,
            ground,
            cat_sprite,
            cat,
            font,
            text
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
        let mut image = image::open(file)?.to_rgba();
        for color in image.pixels_mut() {
            color.0[0] = (color.0[0] as u16 * color.0[3] as u16 / 255) as _;
            color.0[1] = (color.0[1] as u16 * color.0[3] as u16 / 255) as _;
            color.0[2] = (color.0[2] as u16 * color.0[3] as u16 / 255) as _;
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

        self.sprite_program.use_program()?;
        Self::set_transform(&mut self.sprite_program, 1.0 / 360.0, -640.0, 360.0, 1.0, 0.0)?;
        self.font.bind(0)?;
        self.text.render()?;

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
