use nalgebra as na;
use crate::cobalt_engine::{mesh::Mesh, triangle::Vertex, buffer::{EBO, VBO, VAO}};

use super::{camera::{Camera, CameraBuilder, CameraType, camera_builder}, material::Texture};
use gltf::{Gltf, accessor::Iter, material::NormalTexture};
use std::{error::Error, path::PathBuf, str::FromStr};

pub struct SceneDesc {
    pub scene_name: String,
    pub cameras: Vec<Box<dyn Camera>>,
    pub meshes: Vec<Mesh>,
}

impl SceneDesc {
    pub fn new(scene: gltf::Scene, gltf_path: &str, gltf: &gltf::Document, buffers: &Vec<gltf::buffer::Data>, images: &Vec<gltf::image::Data>) -> Result<Self, Box<dyn Error>> {
        let get_buffer_data = |buffer: gltf::Buffer| buffers.get(buffer.index()).map(|x| &*x.0);

        let mut cameras = vec![];
        let mut meshes = vec![];
        
        process_gltf_node(gltf_path, scene.nodes().next().unwrap(), &mut meshes, gltf, buffers);

        if cameras.len() == 0 {
            cameras.push(camera_builder(CameraType::PERSPECTIVE).build());
        }

        Ok(Self {
            scene_name: scene.name().unwrap_or("Default Scene").to_string(),
            cameras,
            meshes
        })
    }
}

pub fn generate_scenes_from_gltf(file_path: &PathBuf) -> Vec<SceneDesc> {
    let mut scenes = vec![];
    let (gltf, buffers, images) = gltf::import(file_path).unwrap();

    for scene in gltf.scenes() {
        scenes.push(SceneDesc::new(scene, file_path.to_str().unwrap(), &gltf, &buffers, &images).unwrap());
    }

    scenes
}

fn process_gltf_node(
    gltf_path: &str,
    node: gltf::Node,
    meshes: &mut Vec<Mesh>,
    gltf: &gltf::Document,
    buffers: &[gltf::buffer::Data],
) -> () {
    if node.mesh().is_some() {
        process_gltf_mesh(gltf_path, &node.mesh().unwrap(), meshes, gltf, buffers);
    }

    for child in node.children() {
        process_gltf_node(gltf_path, child, meshes, gltf, buffers);
    }
}

fn process_gltf_mesh(
    gltf_path: &str,
    mesh: &gltf::Mesh,
    meshes: &mut Vec<Mesh>,
    _gltf: &gltf::Document,
    buffers: &[gltf::buffer::Data],
) -> () {
    for primitive in mesh.primitives() {
        let mut gl_vertices: Vec<f32> = Vec::new();
        let mut gl_normals: Vec<f32> = Vec::new();
        let mut gl_texcoords: Vec<f32> = Vec::new();
        let mut gl_indices: Vec<u32> = Vec::new();
        let mut gl_tangents: Vec<f32> = Vec::new();

        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        let positions = reader.read_positions().unwrap().collect::<Vec<[f32; 3]>>();

        let mut normals = vec![[0.0; 3]; positions.len()];
        if reader.read_normals().is_some() {
            let normals = reader.read_normals().unwrap().collect::<Vec<[f32; 3]>>();
        }

        let indices = reader
            .read_indices()
            .unwrap()
            .clone()
            .into_u32()
            .collect::<Vec<u32>>();

        let mut tangents = vec![[1.0; 4]; positions.len()];

        if reader.read_tangents().is_some() {
            tangents = reader.read_tangents().unwrap().collect::<Vec<[f32; 4]>>();
        }

        for i in 0..positions.len() {
            let position = positions[i];
            let normal = normals[i];
            let tangent = tangents[i];

            // Triangle order swap - gltf uses a different winding order?
            gl_vertices.push(-position[2]); // Flip height
            gl_vertices.push(position[1]);
            gl_vertices.push(position[0]);

            // We're Z-up, so switch Y with Z
            gl_normals.push(normal[0]);
            gl_normals.push(-normal[2]); // Flip height
            gl_normals.push(normal[1]);

            // Flip Y and Z for tangents
            gl_tangents.push(tangent[0]);
            gl_tangents.push(-tangent[2]); // Flip height
            gl_tangents.push(tangent[1]);
        }

        for i in 0..indices.len() {
            gl_indices.push(indices[i]);
        }

        let mesh = Mesh::new(
            gl_vertices,
            gl_indices,
            na::Matrix4::identity(),
        );
        meshes.push(mesh);
    }
}

/*
fn process_gltf_texture(gltf_path: &str, info: Option<gltf::texture::Info>) -> Texture {
    let texture: Texture;

    if info.is_some() {
        // Get image data from buffer view
        let image_source = info.unwrap().texture().source().source();
        match image_source {
            gltf::image::Source::Uri { uri, .. } => {
                let gltf_dir = std::path::Path::new(gltf_path);
                let texture_path = gltf_dir.with_file_name(uri);

                texture = Texture::new(texture_path);
            }
            gltf::image::Source::View { .. } => {
                todo!();
            }
        }
    } else {
        texture = Texture::new(PathBuf::from_str("content/textures/missing.png").unwrap());
    }

    return texture;
}

fn process_gltf_normal_map(gltf_path: &str, normal: Option<NormalTexture>) -> Texture {
    let normal_texture: Texture;

    if normal.is_some() {
        // Get image data from buffer view
        let image_source = normal.unwrap().texture().source().source();
        match image_source {
            gltf::image::Source::Uri { uri, .. } => {
                let gltf_dir = std::path::Path::new(gltf_path);
                let texture_path = gltf_dir.with_file_name(uri);

                normal_texture = Texture::new(texture_path.to_str().unwrap());
            }
            gltf::image::Source::View { .. } => {
                todo!();
            }
        }
    } else {
        normal_texture = Texture::new("content/textures/missing.png");
    }

    return normal_texture;
}

*/