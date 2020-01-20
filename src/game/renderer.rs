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
use text::Text;
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
    open_sans: text::Font,
    roboto: text::Font,
    small_roboto: text::Font,
    text: (Text, Text, Text)
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

        let library = text::Library::new();
        let open_sans = library.new_font("font/OpenSans-Regular.ttf", 48);
        let roboto = library.new_font("font/Roboto-Regular.ttf", 48);
        let small_roboto = library.new_font("font/Roboto-Italic.ttf", 24);
        let mut text = (Text::new(), Text::new(), Text::new());
        let mut p = vec2(10.0, -10.0);
        text.2.add_hb_text(&small_roboto, "OpenSans with FreeType (no kerning):", p);
        p.y -= small_roboto.height();
        text.0.add_text(&open_sans, "Cat Noodle! T,T,T, +/- WAT.", p);
        p.y -= open_sans.height() * 1.5;
        text.2.add_hb_text(&small_roboto, "OpenSans with FreeType + HarfBuzz:", p);
        p.y -= small_roboto.height();
        text.0.add_hb_text(&open_sans, "Cat Noodle! T,T,T, +/- WAT.", p);
        p.y -= open_sans.height() * 1.5;
        text.2.add_hb_text(&small_roboto, "Roboto with FreeType (no kerning):", p);
        p.y -= small_roboto.height();
        text.1.add_text(&roboto, "Cat Noodle! T,T,T, +/- WAT.", p);
        p.y -= roboto.height() * 1.5;
        text.2.add_hb_text(&small_roboto, "Roboto with FreeType + HarfBuzz:", p);
        p.y -= small_roboto.height();
        text.1.add_hb_text(&roboto, "Cat Noodle! T,T,T, +/- WAT.", p);
        text.0.update(false)?;
        text.1.update(false)?;
        text.2.update(false)?;

        Ok(Renderer {
            sprite_program,
            debug_program,
            debug_renderer,
            ground_sprite,
            ground,
            cat_sprite,
            cat,
            open_sans,
            roboto,
            small_roboto,
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
        self.open_sans.bind(0)?;
        self.text.0.render()?;
        self.roboto.bind(0)?;
        self.text.1.render()?;
        self.small_roboto.bind(0)?;
        self.text.2.render()?;

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
