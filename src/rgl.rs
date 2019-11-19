use std::{
    error::Error,
    fmt,
    ptr,
    ffi::CString,
    mem
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

pub enum BufferUsage {
    StreamDraw,
    StaticDraw
}

pub struct VertexBuffer {
    index: GLuint,
    size: usize
}

pub enum AttributeType {
    UnsignedByte,
    Float
}

pub struct VertexArray {
    index: GLuint,
    buffer: VertexBuffer
}


fn handle_error(function: &str) -> Result<(), GLError> {
    if unsafe { gl::GetError() } == gl::NO_ERROR {
        return Ok(());
    }
    while unsafe { gl::GetError() } != gl::NO_ERROR {}
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
        let mut compile_status = gl::FALSE as _;
        unsafe { gl::GetShaderiv(self.index, gl::COMPILE_STATUS, &mut compile_status); }
        handle_error("GetShaderiv")?;
        if compile_status == gl::FALSE as _ {
            let mut log_length = 0;
            unsafe { gl::GetShaderiv(self.index, gl::INFO_LOG_LENGTH, &mut log_length); }
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
        let mut link_status = gl::FALSE as _;
        unsafe { gl::GetProgramiv(self.index, gl::LINK_STATUS, &mut link_status); }
        handle_error("GetProgramiv")?;
        if link_status == gl::FALSE as _ {
            let mut log_length = 0;
            unsafe { gl::GetProgramiv(self.index, gl::INFO_LOG_LENGTH, &mut log_length); }
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


impl VertexBuffer {
    pub fn new() -> Result<VertexBuffer, GLError> {
        let mut index = 0;
        unsafe { gl::GenBuffers(1, &mut index); }
        handle_error("GenBuffers")?;
        Ok(VertexBuffer { index, size: 0 })
    }


    pub fn set_data<T>(&mut self, data: &[T], usage: BufferUsage) -> Result<(), GLError> {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.index); }
        handle_error("BindBuffer")?;
        let usage = match usage {
            BufferUsage::StreamDraw => gl::STREAM_DRAW,
            BufferUsage::StaticDraw => gl::STATIC_DRAW
        };
        unsafe { gl::BufferData(gl::ARRAY_BUFFER, mem::size_of_val(data) as _, data.as_ptr() as _, usage); }
        handle_error("BufferData")?;
        self.size = data.len();
        Ok(())
    }
}


impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.index); }
    }
}


impl VertexArray {
    pub fn new(buffer: VertexBuffer) -> Result<VertexArray, GLError> {
        let mut index = 0;
        unsafe { gl::GenVertexArrays(1, &mut index); }
        handle_error("GenVertexArrays")?;
        Ok(VertexArray { index, buffer })
    }


    pub fn define_attribute(&mut self,
                            index: u32,
                            size: i32,
                            attribute_type: AttributeType,
                            normalized: bool,
                            stride: usize,
                            offset: usize) -> Result<(), GLError>
    {
        unsafe { gl::BindVertexArray(self.index); }
        handle_error("BindVertexArray")?;
        unsafe { gl::EnableVertexAttribArray(index); }
        handle_error("EnableVertexAttribArray")?;
        let attribute_type = match attribute_type {
            AttributeType::UnsignedByte => gl::UNSIGNED_BYTE,
            AttributeType::Float => gl::FLOAT
        };
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer.index); }
        handle_error("BindBuffer")?;
        unsafe { gl::VertexAttribPointer(index, size, attribute_type, normalized as _, stride as _, offset as _); }
        handle_error("VertexAttribPointer")?;
        Ok(())
    }


    pub fn draw(&self) -> Result<(), GLError> {
        unsafe { gl::BindVertexArray(self.index); }
        handle_error("BindVertexArray")?;
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, self.buffer.size as _); }
        handle_error("DrawArrays")?;
        Ok(())
    }
}


impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.index); }
    }
}
