extern crate glfw;
use glfw::Context;

mod rgl;


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

    init_renderer();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_event(event, &mut window);
        }
        render(&glfw);
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


fn init_renderer() {
    init_shaders();
}


fn init_shaders() {
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
    program.use_program().unwrap();
}


fn render(glfw: &glfw::Glfw) {
    let time = glfw.get_time();
    let l = time.sin() as f32 * 0.5 + 0.5;
    rgl::clear(l, l, l, 1.).unwrap();
}
