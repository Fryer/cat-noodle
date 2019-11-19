use std::mem;

extern crate glfw;
use glfw::Context;

use rgl;


struct Position (f32, f32);
struct Color (u8, u8, u8);

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
    let vertex = (Position(0.0, 0.0), Color(0, 0, 0));
    let stride = mem::size_of_val(&vertex);
    let position_offset = &vertex.0 as *const _ as usize - &vertex as *const _ as usize;
    let color_offset = &vertex.1 as *const _ as usize - &vertex as *const _ as usize;

    let mut buffer = rgl::VertexBuffer::new().unwrap();
    let vertices = [
        (Position(-0.5, 0.5), Color(255, 0, 0)),
        (Position(-0.5, -0.5), Color(0, 255, 0)),
        (Position(0.5, -0.5), Color(0, 0, 255))
    ];
    buffer.set_data(&vertices, rgl::BufferUsage::StaticDraw).unwrap();

    let mut sprite = rgl::VertexArray::new(buffer).unwrap();
    sprite.define_attribute(0, 2, rgl::AttributeType::Float, false, stride, position_offset).unwrap();
    sprite.define_attribute(1, 3, rgl::AttributeType::UnsignedByte, true, stride, color_offset).unwrap();

    sprite
}


fn render(glfw: &glfw::Glfw, renderer: &Renderer) {
    let time = glfw.get_time();
    let l = time.sin() as f32 * 0.5 + 0.5;
    rgl::clear(l, l, l, 1.).unwrap();
    renderer.sprite_program.use_program().unwrap();
    renderer.sprite.draw().unwrap();
}
