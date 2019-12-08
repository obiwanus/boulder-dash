use failure::Fail;
use gl;
use gl::types::{GLchar, GLenum, GLint, GLuint};
use std::ffi::CString;
use std::fs;
use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O Error")]
    IoError(#[cause] io::Error),
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Failed to link program: ")]
    LinkError(String),
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::IoError(other)
    }
}

pub struct Program {
    id: GLuint,
    vert: Option<Shader>,
    frag: Option<Shader>,
}

impl Program {
    pub fn new() -> Program {
        let program_id = unsafe { gl::CreateProgram() };
        Program {
            id: program_id,
            vert: None,
            frag: None,
        }
    }

    pub fn vertex_shader(mut self, path: &str) -> Result<Self, Error> {
        let shader = Shader::new(gl::VERTEX_SHADER, path)?;
        unsafe {
            gl::AttachShader(self.id, shader.id());
        }
        self.vert = Some(shader);

        Ok(self)
    }

    pub fn fragment_shader(mut self, path: &str) -> Result<Self, Error> {
        let shader = Shader::new(gl::FRAGMENT_SHADER, path)?;
        unsafe {
            gl::AttachShader(self.id, shader.id());
        }
        self.frag = Some(shader);

        Ok(self)
    }

    pub fn link(self) -> Result<Self, Error> {
        unsafe {
            gl::LinkProgram(self.id);
        }
        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = new_cstring(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    self.id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
            }
            return Err(Error::LinkError(error.to_string_lossy().into_owned()));
        }

        Ok(self)
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
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

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn new_cstring(len: usize) -> CString {
    let buffer: Vec<u8> = vec![0; len];
    unsafe { CString::from_vec_unchecked(buffer) }
}
