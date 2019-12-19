use failure::Fail;
use gl;
use gl::types::*;
use glm::{Mat4, Vec3};
use std::ffi::CString;
use std::fs;
use std::io;

#[derive(Debug, Fail)]
pub enum ShaderError {
    #[fail(display = "I/O Error ({})", name)]
    IoError {
        name: String,
        #[cause]
        inner: io::Error,
    },
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Failed to link program: ")]
    LinkError(String),
    #[fail(display = "Couldn't get uniform location for '{}'", name)]
    UniformLocationNotFound { name: String },
}

pub type Result<T> = std::result::Result<T, ShaderError>;

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

    pub fn vertex_shader(mut self, path: &str) -> Result<Self> {
        let shader = Shader::new(gl::VERTEX_SHADER, path)?;
        unsafe {
            gl::AttachShader(self.id, shader.id());
        }
        self.vert = Some(shader);

        Ok(self)
    }

    pub fn fragment_shader(mut self, path: &str) -> Result<Self> {
        let shader = Shader::new(gl::FRAGMENT_SHADER, path)?;
        unsafe {
            gl::AttachShader(self.id, shader.id());
        }
        self.frag = Some(shader);

        Ok(self)
    }

    pub fn link(self) -> Result<Self> {
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
            return Err(ShaderError::LinkError(error.to_string_lossy().into_owned()));
        }

        Ok(self)
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> Result<GLint> {
        let location = unsafe {
            gl::GetUniformLocation(
                self.id,
                CString::new(name).unwrap().as_ptr() as *const GLchar,
            )
        };
        if location < 0 {
            return Err(ShaderError::UniformLocationNotFound {
                name: name.to_owned(),
            });
        }
        Ok(location)
    }

    /// Assigns a name from the shader to a texture unit
    pub fn set_texture_unit(&self, name: &str, unit: i32) -> Result<()> {
        let location = self.get_uniform_location(name)?;
        unsafe {
            gl::Uniform1i(location, unit);
        }
        Ok(())
    }

    /// Sets a vec3 uniform
    pub fn set_vec3(&self, name: &str, vec: Vec3) -> Result<()> {
        let location = self.get_uniform_location(name)?;
        unsafe {
            gl::Uniform3fv(location, 1, vec.as_ptr());
        }
        Ok(())
    }

    /// Sets a vec3 uniform
    pub fn set_mat4(&self, name: &str, mat: &Mat4) -> Result<()> {
        let location = self.get_uniform_location(name)?;
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr());
        }
        Ok(())
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
    pub fn new(kind: GLenum, path: &str) -> Result<Self> {
        let source = fs::read_to_string(path).map_err(|e| ShaderError::IoError {
            name: path.to_owned(),
            inner: e,
        })?;
        let source = CString::new(source).unwrap();
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
            return Err(ShaderError::CompileError {
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
