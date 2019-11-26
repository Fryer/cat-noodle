use std::{
    panic,
    ptr,
    ffi::CStr,
    thread,
    sync::{
        Arc,
        atomic::{
            AtomicBool,
            Ordering
        }
    }
};

extern crate glfw;
use glfw::Context;

mod renderer;
use renderer::Renderer;


fn main() {
    let thread_panicked = Arc::new(AtomicBool::new(false));
    let thread_panicked_ref = thread_panicked.clone();
    let default_panic = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        default_panic(info);
        thread_panicked_ref.store(true, Ordering::Relaxed);
    }));

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::DepthBits(Some(0)));
    glfw.window_hint(glfw::WindowHint::StencilBits(Some(0)));

    let (mut window, events) = glfw.create_window(1280, 720, "CAT NOODLE!", glfw::WindowMode::Windowed).unwrap();
    window.set_key_polling(true);

    window.make_current();
    gl::load_with(|p| window.get_proc_address(p) as *const _);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    glfw.make_context_current(None);
    handle_glfw_error();

    let end_game = Arc::new(AtomicBool::new(false));
    let end_game_ref = end_game.clone();
    let mut context = window.render_context();
    let game_thread = thread::Builder::new().name("Game".to_string()).spawn(move || {
        context.make_current();
        handle_glfw_error();
        let mut renderer = Renderer::new().unwrap();
        while !end_game_ref.load(Ordering::Relaxed) {
            renderer.render().unwrap();
            context.swap_buffers();
            handle_glfw_error();
        }
    }).unwrap();

    while !window.should_close() && !thread_panicked.load(Ordering::Relaxed) {
        glfw.wait_events_timeout(0.1);
        for (_, event) in glfw::flush_messages(&events) {
            handle_event(event, &mut window);
        }
        handle_glfw_error();
    }

    end_game.store(true, Ordering::Relaxed);
    game_thread.join().unwrap();
}


fn handle_glfw_error() {
    let mut description = ptr::null();
    let error = unsafe { glfw::ffi::glfwGetError(&mut description) };
    if error != glfw::ffi::NO_ERROR {
        let description = unsafe { CStr::from_ptr(description) };
        panic!("GLFW error: {}", description.to_str().unwrap());
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
