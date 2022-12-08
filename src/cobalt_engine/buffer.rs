use std::{mem::size_of_val, ffi::c_void};

use gl::{ARRAY_BUFFER, STATIC_DRAW, types::{GLuint, GLsizeiptr, GLfloat}};

use super::triangle::{Vertex};

pub struct VBO {
    pub id: gl::types::GLuint,
}

impl VBO {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self {
            id
        }
    }

    pub fn bind_buffer(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        };
    }

    pub fn unbind_buffer(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        };
    }

    pub fn upload_data(&self, data: &Vec<GLfloat>) {
        self.bind_buffer();
        unsafe {
            gl::BufferData(
                ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const GLfloat as *const c_void,
                STATIC_DRAW,
            );
        }
        self.unbind_buffer();
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct VAO {
    pub id: gl::types::GLuint,
}

impl VAO {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Self {
            id
        }
    }

    pub fn bind_buffer(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        };
    }

    pub fn unbind_buffer(&self) {
        unsafe {
            gl::BindVertexArray(0);
        };
    }

    pub fn setup_buffer(&self, vbo: &VBO) {
        self.bind_buffer();
        vbo.bind_buffer();
        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<GLfloat>()) as gl::types::GLint,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(0);
        }
        self.unbind_buffer()
    }
}

pub struct EBO {
    id: gl::types::GLuint
}

impl EBO {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self {
            id
        }
    }

    pub fn bind_buffer(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        };
    }

    pub fn unbind_buffer(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        };
    }

    pub fn upload_data(&self, indices: &Vec<GLuint>) {
        self.bind_buffer();
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                &indices[0] as *const GLuint as *const c_void,
                gl::STATIC_DRAW,
            );
        }
        self.unbind_buffer();
    }
}

impl Drop for EBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}