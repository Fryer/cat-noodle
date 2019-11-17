use std::ptr;
use std::ffi::CString;

extern crate gl;
use gl::types::*;


pub enum ShaderType {
    Vertex,
    Fragment
}

pub struct Shader {
    index: GLuint
}

pub struct Program {
    index: GLuint
}


fn has_error() -> bool {
    if unsafe { gl::GetError() } == gl::NO_ERROR {
        return false;
    }
    while unsafe { gl::GetError() } != gl::NO_ERROR {}
    true
}


pub fn clear(r: f32, g: f32, b: f32, a: f32) -> Result<(), String> {
    unsafe { gl::ClearColor(r, g, b, a); }
    if has_error() {
        return Err(String::from("ClearColor error."));
    }
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
    if has_error() {
        return Err(String::from("Clear error."));
    }
    Ok(())
}


impl Shader {
    pub fn new(shader_type: ShaderType) -> Result<Shader, String> {
        let shader_type = match shader_type {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER
        };
        let shader = unsafe { gl::CreateShader(shader_type) };
        if has_error() {
            return Err(String::from("CreateShader error."));
        }
        Ok(Shader { index: shader })
    }


    pub fn set_source(&mut self, source: &str) -> Result<(), String> {
        let source = CString::new(source).unwrap();
        unsafe { gl::ShaderSource(self.index, 1, &source.as_ptr(), ptr::null()); }
        if has_error() {
            return Err(String::from("ShaderSource error."));
        }
        Ok(())
    }


    pub fn compile(&mut self) -> Result<(), String> {
        unsafe { gl::CompileShader(self.index); }
        if has_error() {
            return Err(String::from("CompileShader error."));
        }
        let mut compile_status = gl::FALSE as GLint;
        unsafe { gl::GetShaderiv(self.index, gl::COMPILE_STATUS, &mut compile_status as *mut _); }
        if compile_status == gl::FALSE as GLint {
            let mut log_length = 0;
            unsafe { gl::GetShaderiv(self.index, gl::INFO_LOG_LENGTH, &mut log_length as *mut _); }
            if log_length > 0 {
                let mut log = vec![0; log_length as usize];
                let log_ptr = log.as_mut_ptr();
                unsafe { gl::GetShaderInfoLog(self.index, log_length, ptr::null_mut(), log_ptr); }
                let log = unsafe { CString::from_raw(log_ptr) }.into_string().unwrap();
                return Err(format!("Shader compilation error:\n{}", log));
            }
            return Err(String::from("Shader compilation error."));
        }
        Ok(())
    }
}


impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.index); }
    }
}


impl Program {
    pub fn new() -> Result<Program, String> {
        let program = unsafe { gl::CreateProgram() };
        if has_error() {
            return Err(String::from("CreateProgram error."));
        }
        Ok(Program { index: program })
    }


    pub fn attach_shader(&mut self, shader: &Shader) -> Result<(), String> {
        unsafe { gl::AttachShader(self.index, shader.index); }
        if has_error() {
            return Err(String::from("AttachShader error."));
        }
        Ok(())
    }


    pub fn link(&mut self) -> Result<(), String> {
        unsafe { gl::LinkProgram(self.index); }
        if has_error() {
            return Err(String::from("LinkProgram error."));
        }
        let mut link_status = gl::FALSE as GLint;
        unsafe { gl::GetProgramiv(self.index, gl::LINK_STATUS, &mut link_status as *mut _); }
        if link_status == gl::FALSE as GLint {
            let mut log_length = 0;
            unsafe { gl::GetProgramiv(self.index, gl::INFO_LOG_LENGTH, &mut log_length as *mut _); }
            if log_length > 0 {
                let mut log = vec![0; log_length as usize];
                let log_ptr = log.as_mut_ptr();
                unsafe { gl::GetProgramInfoLog(self.index, log_length, ptr::null_mut(), log_ptr); }
                let log = unsafe { CString::from_raw(log_ptr) }.into_string().unwrap();
                return Err(format!("Program link error:\n{}", log));
            }
            return Err(String::from("Program link error."));
        }
        Ok(())
    }


    pub fn use_program(&self) -> Result<(), String> {
        unsafe { gl::UseProgram(self.index); }
        if has_error() {
            return Err(String::from("UseProgram error."));
        }
        Ok(())
    }
}


impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.index); }
    }
}
