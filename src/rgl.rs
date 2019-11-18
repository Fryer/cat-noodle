use std::{
    error::Error,
    fmt,
    ptr,
    ffi::CString
};

extern crate gl;
use gl::types::*;


pub struct GLError {
    error: String
}

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


fn handle_error(function: &str) -> Result<(), GLError> {
    let mut error = unsafe { gl::GetError() };
    if error == gl::NO_ERROR {
        return Ok(());
    }
    while error != gl::NO_ERROR {
        match error {
            gl::OUT_OF_MEMORY => return Err(GLError { error: format!("{} failed (OUT_OF_MEMORY)", function) }),
            gl::CONTEXT_LOST => return Ok(()),
            _ => {}
        }
        error = unsafe { gl::GetError() };
    }
    Err(GLError { error: format!("{} failed", function) })
}


pub fn clear(r: f32, g: f32, b: f32, a: f32) -> Result<(), GLError> {
    unsafe { gl::ClearColor(r, g, b, a); }
    handle_error("ClearColor")?;
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
    handle_error("Clear")?;
    Ok(())
}


impl Error for GLError {}


impl fmt::Debug for GLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}


impl fmt::Display for GLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}


impl Shader {
    pub fn new(shader_type: ShaderType) -> Result<Shader, GLError> {
        let shader_type = match shader_type {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER
        };
        let shader = unsafe { gl::CreateShader(shader_type) };
        handle_error("CreateShader")?;
        Ok(Shader { index: shader })
    }


    pub fn set_source(&mut self, source: &str) -> Result<(), GLError> {
        let source = CString::new(source).unwrap();
        unsafe { gl::ShaderSource(self.index, 1, &source.as_ptr(), ptr::null()); }
        handle_error("ShaderSource")?;
        Ok(())
    }


    pub fn compile(&mut self) -> Result<(), GLError> {
        unsafe { gl::CompileShader(self.index); }
        handle_error("CompileShader")?;
        let mut compile_status = gl::FALSE as GLint;
        unsafe { gl::GetShaderiv(self.index, gl::COMPILE_STATUS, &mut compile_status as *mut _); }
        handle_error("GetShaderiv")?;
        if compile_status == gl::FALSE as GLint {
            let mut log_length = 0;
            unsafe { gl::GetShaderiv(self.index, gl::INFO_LOG_LENGTH, &mut log_length as *mut _); }
            handle_error("GetShaderiv")?;
            if log_length > 0 {
                let mut log = vec![0; log_length as usize];
                let log_ptr = log.as_mut_ptr();
                unsafe { gl::GetShaderInfoLog(self.index, log_length, ptr::null_mut(), log_ptr); }
                handle_error("GetShaderInfoLog")?;
                let log = unsafe { CString::from_raw(log_ptr) }.into_string().unwrap();
                return Err(GLError { error: format!("compilation failed:\n{}", log) });
            }
            return Err(GLError { error: String::from("compilation failed") });
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
    pub fn new() -> Result<Program, GLError> {
        let program = unsafe { gl::CreateProgram() };
        handle_error("CreateProgram")?;
        Ok(Program { index: program })
    }


    pub fn attach_shader(&mut self, shader: &Shader) -> Result<(), GLError> {
        unsafe { gl::AttachShader(self.index, shader.index); }
        handle_error("AttachShader")?;
        Ok(())
    }


    pub fn link(&mut self) -> Result<(), GLError> {
        unsafe { gl::LinkProgram(self.index); }
        handle_error("LinkProgram")?;
        let mut link_status = gl::FALSE as GLint;
        unsafe { gl::GetProgramiv(self.index, gl::LINK_STATUS, &mut link_status as *mut _); }
        handle_error("GetProgramiv")?;
        if link_status == gl::FALSE as GLint {
            let mut log_length = 0;
            unsafe { gl::GetProgramiv(self.index, gl::INFO_LOG_LENGTH, &mut log_length as *mut _); }
            handle_error("GetProgramiv")?;
            if log_length > 0 {
                let mut log = vec![0; log_length as usize];
                let log_ptr = log.as_mut_ptr();
                unsafe { gl::GetProgramInfoLog(self.index, log_length, ptr::null_mut(), log_ptr); }
                handle_error("GetProgramInfoLog")?;
                let log = unsafe { CString::from_raw(log_ptr) }.into_string().unwrap();
                return Err(GLError { error: format!("LinkProgram failed:\n{}", log) });
            }
            return Err(GLError { error: String::from("LinkProgram failed") });
        }
        Ok(())
    }


    pub fn use_program(&self) -> Result<(), GLError> {
        unsafe { gl::UseProgram(self.index); }
        handle_error("UseProgram")?;
        Ok(())
    }
}


impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.index); }
    }
}
