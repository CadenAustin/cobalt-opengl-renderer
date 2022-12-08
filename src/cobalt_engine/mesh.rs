use std::{ptr, ffi::c_void};

use gl::types::{GLuint, GLfloat, GLint, GLsizeiptr, GLsizei};
use nalgebra as na;

use super::{buffer::{VBO, EBO, VAO}, triangle::Vertex, program::{self, Program, UniformEnum}};

pub struct Mesh {
    pub vbo: VBO,
    pub vao: VAO,
    pub ebo: EBO,
    pub vertex_count: GLint,
    pub index_count: GLint,
    pub model_matrix: na::Matrix4<f32>,
}

impl Mesh {
    pub fn new(
        vertices: Vec<GLfloat>,
        indices: Vec<GLuint>,
        model_matrix: na::Matrix4<f32>,
    ) -> Mesh {
        let vao = VAO::new();
        let vbo = VBO::new();
        vbo.bind_buffer();
        vao.bind_buffer();
        
        let mut gl_data: Vec<GLfloat> = Vec::new();
            for i in 0..(vertices.len() / 3) {
                gl_data.push(vertices[i * 3]);
                gl_data.push(vertices[i * 3 + 1]);
                gl_data.push(vertices[i * 3 + 2]);
            }
        println!("{:?}", gl_data);

        unsafe {
            // Buffer data
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (gl_data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                &gl_data[0] as *const GLfloat as *const c_void,
                gl::STATIC_DRAW,
            );

            let stride = ((3) * std::mem::size_of::<GLfloat>()) as GLsizei;

            // Attributes
            // Position
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
        }

        let ebo = EBO::new();
        ebo.bind_buffer();
        ebo.upload_data(&indices);
        ebo.unbind_buffer();
        vbo.unbind_buffer();
        vao.unbind_buffer();

        Self {
            vbo,
            vao,
            ebo,
            vertex_count: vertices.len() as GLint,
            index_count: indices.len() as GLint,
            model_matrix,
        }
    }

    pub fn render(&self, program: &Program) {
        program.insert_uniform(
            "model_matrix",
            UniformEnum::Matrix4(self.model_matrix),
        );

        self.vao.bind_buffer();
        self.vbo.bind_buffer();
        self.ebo.bind_buffer();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            )
        }
        self.ebo.unbind_buffer();
        self.vao.unbind_buffer();
        self.vbo.unbind_buffer();
    }
}