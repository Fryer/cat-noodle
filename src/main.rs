use std::mem;

extern crate glfw;
use glfw::Context;

extern crate image;

use rgl;


struct Position(f32, f32);
struct TexCoord(f32, f32);
struct Color(u8, u8, u8);

struct Renderer {
    sprite_program: rgl::Program,
    sprite: rgl::VertexArray
}


fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::DepthBits(Some(0)));
    glfw.window_hint(glfw::WindowHint::StencilBits(Some(0)));

    let (mut window, events) = glfw.create_window(1280, 720, "CAT NOODLE!", glfw::WindowMode::Windowed).unwrap();
    window.set_key_polling(true);

    gl::load_with(|p| window.get_proc_address(p) as *const _);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let renderer = init_renderer();

    let cat_image = image::open("img/cat.png").unwrap().to_rgba();
    let cat_width = cat_image.width();
    let cat_height = cat_image.height();
    let cat_data = cat_image.into_raw();
    unsafe { rgl::test_texture(&renderer.sprite_program, cat_data.as_ptr(), cat_width as _, cat_height as _); }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_event(event, &mut window);
        }
        render(&glfw, &renderer);
        window.swap_buffers();
    }
}


fn handle_event(event: glfw::WindowEvent, window: &mut glfw::Window) {
    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}


fn init_renderer() -> Renderer {
    Renderer {
        sprite_program: init_shaders(),
        sprite: init_sprites()
    }
}


fn init_shaders() -> rgl::Program {
    let mut vertex_shader = rgl::Shader::new(rgl::ShaderType::Vertex).unwrap();
    let source = include_str!("sprite.vert");
    vertex_shader.set_source(source).unwrap();
    vertex_shader.compile().unwrap();

    let mut fragment_shader = rgl::Shader::new(rgl::ShaderType::Fragment).unwrap();
    let source = include_str!("sprite.frag");
    fragment_shader.set_source(source).unwrap();
    fragment_shader.compile().unwrap();

    let mut program = rgl::Program::new().unwrap();
    program.attach_shader(&vertex_shader).unwrap();
    program.attach_shader(&fragment_shader).unwrap();
    program.link().unwrap();

    program
}


fn init_sprites() -> rgl::VertexArray {
    let vertex = (Position(0.0, 0.0), TexCoord(0.0, 0.0), Color(0, 0, 0));
    let stride = mem::size_of_val(&vertex);
    let position_offset = &vertex.0 as *const _ as usize - &vertex as *const _ as usize;
    let texcoord_offset = &vertex.1 as *const _ as usize - &vertex as *const _ as usize;
    let color_offset = &vertex.2 as *const _ as usize - &vertex as *const _ as usize;

    let mut buffer = rgl::VertexBuffer::new().unwrap();
    let w = 4.5 / 16.0;
    let h = 0.5;
    let vertices = [
        (Position(-w, h), TexCoord(0.0, 0.0), Color(255, 255, 255)),
        (Position(-w, -h), TexCoord(0.0, 1.0), Color(255, 255, 255)),
        (Position(w, -h), TexCoord(1.0, 1.0), Color(255, 255, 255)),
        (Position(-w, h), TexCoord(0.0, 0.0), Color(255, 255, 255)),
        (Position(w, -h), TexCoord(1.0, 1.0), Color(255, 255, 255)),
        (Position(w, h), TexCoord(1.0, 0.0), Color(255, 255, 255))
    ];
    buffer.set_data(&vertices, rgl::BufferUsage::StaticDraw).unwrap();

    let mut sprite = rgl::VertexArray::new(buffer).unwrap();
    sprite.define_attribute(0, 2, rgl::AttributeType::Float, false, stride, position_offset).unwrap();
    sprite.define_attribute(1, 2, rgl::AttributeType::Float, false, stride, texcoord_offset).unwrap();
    sprite.define_attribute(2, 3, rgl::AttributeType::UnsignedByte, true, stride, color_offset).unwrap();

    sprite
}


fn render(glfw: &glfw::Glfw, renderer: &Renderer) {
    let time = glfw.get_time();
    let l = time.sin() as f32 * 0.1 + 0.2;
    rgl::clear(l, l, l, 1.).unwrap();
    renderer.sprite_program.use_program().unwrap();
    renderer.sprite.draw().unwrap();
}
