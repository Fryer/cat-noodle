extern crate glfw;
extern crate gl;

use std::ptr;
use std::ffi::CString;
use glfw::Context;
use gl::types::*;


fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(1280, 720, "CAT NOODLE!", glfw::WindowMode::Windowed).unwrap();
    window.set_resizable(false);
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
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    assert_ne!(vertex_shader, 0);
    let source = CString::new("#version 400\nvoid main() {}\n").unwrap();
    unsafe { gl::ShaderSource(vertex_shader, 1, &source.as_ptr(), ptr::null()); }
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
    unsafe { gl::CompileShader(vertex_shader); }
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
    let mut compile_status = gl::FALSE as GLint;
    unsafe { gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut compile_status as *mut _); }
    if compile_status == gl::FALSE as GLint {
        let mut log_length = 0;
        unsafe { gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut log_length as *mut _); }
        if log_length > 0 {
            let mut log = vec![0; log_length as usize];
            let log_ptr = log.as_mut_ptr();
            unsafe { gl::GetShaderInfoLog(vertex_shader, log_length, ptr::null_mut(), log_ptr); }
            let log = unsafe { CString::from_raw(log_ptr) }.into_string().unwrap();
            panic!("Vertex shader compilation error:\n{}", log);
        }
        panic!("Vertex shader compilation error.");
    }

    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    assert_ne!(fragment_shader, 0);
    let source = CString::new("#version 400\nvoid main() {}\n").unwrap();
    unsafe { gl::ShaderSource(fragment_shader, 1, &source.as_ptr(), ptr::null()); }
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
    unsafe { gl::CompileShader(fragment_shader); }
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
    let mut compile_status = gl::FALSE as GLint;
    unsafe { gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut compile_status as *mut _); }
    if compile_status == gl::FALSE as GLint {
        let mut log_length = 0;
        unsafe { gl::GetShaderiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut log_length as *mut _); }
        if log_length > 0 {
            let mut log = vec![0; log_length as usize];
            let log_ptr = log.as_mut_ptr();
            unsafe { gl::GetShaderInfoLog(fragment_shader, log_length, ptr::null_mut(), log_ptr); }
            let log = unsafe { CString::from_raw(log_ptr) }.into_string().unwrap();
            panic!("Fragment shader compilation error:\n{}", log);
        }
        panic!("Fragment shader compilation error.");
    }

    let program = unsafe { gl::CreateProgram() };
    assert_ne!(program, 0);
    unsafe { gl::AttachShader(program, vertex_shader); }
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
    unsafe { gl::AttachShader(program, fragment_shader); }
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
    unsafe { gl::LinkProgram(program); }
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
    let mut link_status = gl::FALSE as GLint;
    unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut link_status as *mut _); }
    if link_status == gl::FALSE as GLint {
        let mut log_length = 0;
        unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length as *mut _); }
        if log_length > 0 {
            let mut log = vec![0; log_length as usize];
            let log_ptr = log.as_mut_ptr();
            unsafe { gl::GetProgramInfoLog(program, log_length, ptr::null_mut(), log_ptr); }
            let log = unsafe { CString::from_raw(log_ptr) }.into_string().unwrap();
            panic!("Shader program link error:\n{}", log);
        }
        panic!("Shader program link error.");
    }

    unsafe { gl::DeleteShader(vertex_shader); }
    unsafe { gl::DeleteShader(fragment_shader); }

    unsafe { gl::UseProgram(program); }
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
}


fn render(glfw: &glfw::Glfw) {
    let time = glfw.get_time();
    let l = time.sin() as f32 * 0.5 + 0.5;
    unsafe { gl::ClearColor(l, l, l, 1.); }
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
}
