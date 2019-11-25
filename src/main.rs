extern crate glfw;
use glfw::Context;

mod renderer;
use renderer::Renderer;


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

    let mut renderer = Renderer::new().unwrap();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_event(event, &mut window);
        }
        renderer.render().unwrap();
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
