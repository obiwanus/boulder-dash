use gl::types::*;

pub struct VertexBuffer {
    id: GLuint,
    num_vertices: usize,
}

impl VertexBuffer {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        VertexBuffer {
            id,
            num_vertices: 0,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn set_static_data(mut self, vertex_data: &Vec<f32>, stride: usize) -> Self {
        self.bind();
        self.num_vertices = vertex_data.len() / stride;
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_data.len() * std::mem::size_of::<f32>()) as isize,
                vertex_data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
        self.unbind();
        self
    }

    pub fn draw_triangles(&self, vao: &VertexArray) {
        vao.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.num_vertices as i32);
        }
        vao.unbind();
    }
}

pub struct VertexArray {
    id: GLuint,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VertexArray { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn set_attrib(self, location: u32, count: i32, stride: usize, offset: usize) -> Self {
        self.bind();
        unsafe {
            gl::VertexAttribPointer(
                location,
                count,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as i32,
                (offset * std::mem::size_of::<f32>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(location);
        }
        self.unbind();
        self
    }
}
