use std::{
    panic,
    ptr,
    ffi::CStr,
    thread,
    sync::{mpsc, Mutex}
};

extern crate glfw;
use glfw::Context;

mod game;
use game::Game;


fn main() {
    let (panic_sender, panic_receiver) = mpsc::channel();
    let panic_sender = Mutex::new(panic_sender);
    let default_panic = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        default_panic(info);
        panic_sender.lock().unwrap().send(()).ok();
        unsafe { glfw::ffi::glfwPostEmptyEvent(); }
    }));

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
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

    let (event_sender, event_receiver) = mpsc::channel();
    let mut context = window.render_context();
    let game_thread = thread::Builder::new().name("Game".to_string()).spawn(move || {
        context.make_current();
        handle_glfw_error();
        let mut game = Game::new(event_receiver).unwrap();
        while game.update().unwrap() {
            context.swap_buffers();
            handle_glfw_error();
        }
    }).unwrap();

    while !window.should_close() && panic_receiver.try_recv().is_err() {
        glfw.wait_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_event(event, &mut window, &event_sender);
        }
        handle_glfw_error();
    }

    event_sender.send(game::Event::Close).ok();
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


fn handle_event(event: glfw::WindowEvent, window: &mut glfw::Window, sender: &mpsc::Sender<game::Event>) {
    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(key, _, glfw::Action::Press, _) => {
            sender.send(game::Event::Key(glfw::Action::Press, key)).ok();
        }
        glfw::WindowEvent::Key(key, _, glfw::Action::Release, _) => {
            sender.send(game::Event::Key(glfw::Action::Release, key)).ok();
        }
        _ => {}
    }
}
