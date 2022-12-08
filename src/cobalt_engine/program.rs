use nalgebra as na;
use gl;

use gl::types::{GLint, GLfloat};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::{path::PathBuf, error::Error};
use std::ffi::CString;

pub enum UniformEnum {
    Integer(GLint),
    Float(GLfloat),
    Matrix4(na::Matrix4<f32>)
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(file_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let shader_type = match file_path.extension() {
            Some(os_str) => match os_str.to_str() {
                Some("vs") => gl::VERTEX_SHADER,
                Some("fs") => gl::FRAGMENT_SHADER,
                None | _ => panic!("Shader Type is not Valid"),
            },
            None => panic!("Shader File Extension is Unreadable"),
        };

        let shader_str = read_file_to_c_str(file_path);
        let id = unsafe {
            gl::CreateShader(shader_type)
        };
        
        unsafe {
            gl::ShaderSource(id, 1, &shader_str.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        Ok(
            Self {
                id,
            }
        )
    }

    pub fn id(&self) -> gl::types::GLuint {
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

pub struct Program {
    id: gl::types::GLuint,
    uniform_locations: RefCell<HashMap<String, GLint>>,
}

impl Program {
    pub fn from_shaders(shaders: Vec<Shader>) -> Result<Self, Box<dyn Error>> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }

        Ok(
            Self {
                id,
                uniform_locations: RefCell::new(HashMap::new()),
            }
        )
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn insert_uniform(&self, name: &str, val: UniformEnum) {
        if let Some(uniform_idx) = self.get_uniform(name) {
            unsafe {
                match val {
                    UniformEnum::Integer(i) => {
                        gl::Uniform1i(uniform_idx, i)
                    },
                    UniformEnum::Float(f) => {
                        gl::Uniform1f(uniform_idx, f)
                    },
                    UniformEnum::Matrix4(mat) => {
                        gl::ProgramUniformMatrix4fv(self.id, uniform_idx, 1, gl::FALSE, mat.as_slice().as_ptr())
                    },
                }
            }
        }

    }

    pub fn get_uniform(&self, name: &str) -> Option<GLint> {
        if let Some(uniform_idx) = self.uniform_locations.borrow().get(name) {
            return Some(*uniform_idx);
        }

        let c_name = CString::new(name).unwrap();
        let uniform_idx = unsafe {
            gl::GetUniformLocation(self.id(), c_name.as_ptr())
        };

        if uniform_idx != -1 {
            self.uniform_locations.borrow_mut().insert(name.to_owned(), uniform_idx);
            Some(uniform_idx)
        } else {
            None
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



pub fn read_file_to_c_str(file_path: &PathBuf) -> CString {
    let contents = fs::read_to_string(file_path).expect("Shader File Unreadable");
    CString::new(contents).unwrap()
}