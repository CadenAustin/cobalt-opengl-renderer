use std::path::PathBuf;

use glam::{Vec3, vec3};
use miniquad::{gl::GLuint, Context, Texture, TextureParams};
use tinytga::{Tga, RawTga};
use std::fs;
use image::open;


#[derive(Clone, Debug)]
pub enum LightingType {
    FLAT,
    SPECULAR,
}


#[derive(Clone, Debug, Copy)]
pub struct TextureDesc {
    pub texture: Texture,
    pub gl_id: GLuint,
}

impl TextureDesc {
    pub fn from_file(ctx: &mut Context, file_path: PathBuf) -> Option<Self> {
        match &file_path.extension().unwrap().to_str().unwrap() {
            &"tga" => TextureDesc::from_tga(ctx, file_path),
            &"jpg" | &"png" => TextureDesc::from_image(ctx, file_path),
            _ => None
        }
    
    }

    pub fn from_tga(ctx: &mut Context, file_path: PathBuf) -> Option<Self> {
        let data = std::fs::read(file_path).unwrap();
        let img = RawTga::from_slice(&data).unwrap();
        let headers = img.header();
        let format = match headers.pixel_depth {
            tinytga::Bpp::Bits8 => todo!(),
            tinytga::Bpp::Bits16 => todo!(),
            tinytga::Bpp::Bits24 => miniquad::TextureFormat::RGB8,
            tinytga::Bpp::Bits32 => miniquad::TextureFormat::RGBA8,
            _ => todo!(),
        };
        let bytes: Vec<u8> = img.image_data().to_vec();
        let texture = Texture::from_data_and_format(ctx, &bytes, TextureParams {
            format,
            wrap: miniquad::TextureWrap::Clamp,
            filter: miniquad::FilterMode::Linear,
            width: headers.width as u32,
            height: headers.height as u32,
        });
        
        Some(Self {
            texture,
            gl_id: texture.gl_internal_id(),
        })
    }

    pub fn from_image(ctx: &mut Context, file_path: PathBuf) -> Option<Self> {
        let img = open(file_path).unwrap().into_rgba8();
        let (width, height) = img.dimensions();
        let bytes = img.into_raw();
        let texture = Texture::from_data_and_format(ctx, &bytes, TextureParams {
            format: miniquad::TextureFormat::RGBA8,
            wrap: miniquad::TextureWrap::Clamp,
            filter: miniquad::FilterMode::Linear,
            width: width as u32,
            height: height as u32,
        });
        
        Some(Self {
            texture,
            gl_id: texture.gl_internal_id(),
        })
    }

    pub fn from_png(ctx: &mut Context, file_path: PathBuf) -> Option<Self> {
        println!("{:?}", file_path);
        let data = std::fs::read(file_path).unwrap();
        let img = RawTga::from_slice(&data).unwrap();
        let headers = img.header();
        let bytes: Vec<u8> = img.image_data().to_vec();
        let texture = Texture::from_data_and_format(ctx, &bytes, TextureParams {
            format: miniquad::TextureFormat::RGBA8,
            wrap: miniquad::TextureWrap::Clamp,
            filter: miniquad::FilterMode::Linear,
            width: headers.width as u32,
            height: headers.height as u32,
        });
        
        Some(Self {
            texture,
            gl_id: texture.gl_internal_id(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct MaterialDesc {
    pub ambient_color: Vec3,
    pub diffuse_color: Vec3,
    pub specular_color: Vec3,
    pub transparancy: f32,
    pub shininess: f32,
    pub lighting_type: LightingType,
    pub ambient_map: Option<TextureDesc>,
    pub diffuse_map: Option<TextureDesc>,
    pub specular_map: Option<TextureDesc>,
}

impl Default for MaterialDesc {
    fn default() -> Self {
        Self {
            ambient_color: vec3(0.2, 0.2, 0.2),
            diffuse_color: vec3(0.8, 0.8, 0.8),
            specular_color: vec3(1.0, 1.0, 1.0),
            transparancy: 1.0,
            shininess: 0.0,
            lighting_type: LightingType::FLAT,
            ambient_map: None,
            diffuse_map: None,
            specular_map: None,
        }
    }
}
