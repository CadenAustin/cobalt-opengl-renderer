use std::fs;

use miniquad::ShaderMeta;

pub struct ShaderDesc {
    pub vert: String,
    pub frag: String,
    pub meta: ShaderMeta
}

impl ShaderDesc {
    pub fn from_file(vert_file_name: &str, frag_file_name: &str, meta: ShaderMeta) -> Result<Self, Box<dyn std::error::Error>> {
        let vert = fs::read_to_string(vert_file_name).expect("Error reading Vertex Shader.").to_string();
        let frag = fs::read_to_string(frag_file_name).expect("Error reading Fragment Shader.").to_string();

        Ok(Self { vert, frag, meta })
    }
}