use failure::Fail;
use gl;
use gl::types::{GLchar, GLenum, GLint, GLuint};
use std::ffi::CString;
use std::fs;
use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load shader {}: {}", name, error)]
    IoError(#[cause] io::Error),
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Failed to link program: ")]
    LinkError(String),
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Error {
        Error::IoError(other)
    }
}

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn new() -> Program {
        let program_id = unsafe { gl::CreateProgram() };
        Program { id: program_id }
    }

    pub fn vertex_shader(self, path: &str) -> Self {
        self
    }
}

struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new(kind: GLenum, path: &str) -> Result<Self, Error> {
        let source = CString::new(fs::read_to_string(path)?).unwrap();
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }
        let mut success: GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = new_cstring(len as usize);
            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }
            return Err(Error::CompileError {
                name: path.to_owned(),
                message: error.to_string_lossy().into_owned(),
            });
        }

        Ok(Shader { id })
    }
}

fn new_cstring(len: usize) -> CString {
    let buffer: Vec<u8> = vec![0; len];
    unsafe { CString::from_vec_unchecked(buffer) }
}
