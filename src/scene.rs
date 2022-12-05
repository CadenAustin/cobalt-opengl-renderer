use glam::{vec3, Vec3};
use miniquad::{Bindings, Buffer, BufferType, Context};
use tobj::{Material, Mesh, Model};
use crate::material::{TextureDesc, MaterialDesc, LightingType};

use std::{
    sync::atomic::{AtomicUsize, Ordering}, collections::HashMap, path::{PathBuf, Path},
};
fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone)]
pub struct ObjDesc {
    pub name: String,
    pub meshes: Vec<MeshDesc>,
    pub materials: Vec<Material>,
}

impl ObjDesc {
    pub fn from_obj_file(ctx: &mut Context, obj_file: PathBuf) -> Self {
        let mut meshes: Vec<MeshDesc> = vec![];
        let mut vertex_buffers: Vec<Buffer> = vec![];
        let mut index_buffers: Vec<Buffer> = vec![];

        let name = obj_file.file_name().unwrap().to_str().unwrap().replace(".obj", "");

        let obj = tobj::load_obj(obj_file, &tobj::GPU_LOAD_OPTIONS);
        let (models, materials) = obj.expect("Failed to load OBJ file");

        let materials = materials.unwrap_or(vec![]);

        let mut mat_descs: HashMap<usize, MaterialDesc> = HashMap::new();
        for (mat_idx, mat) in materials.iter().enumerate() {
            let lighting_type = match mat.illumination_model {
                Some(1) => LightingType::FLAT,
                Some(2) => LightingType::SPECULAR,
                None => LightingType::FLAT,
                _ => LightingType::FLAT,
            };

            let base_dir = Path::new("models/");

            let ambient_map = if (mat.ambient_texture != "") {TextureDesc::from_file(ctx, base_dir.join(std::path::Path::new(&mat.ambient_texture).to_path_buf()))} else {None};
            let diffuse_map = if (mat.diffuse_texture != "") {TextureDesc::from_file(ctx, base_dir.join(std::path::Path::new(&mat.diffuse_texture).to_path_buf()))} else {None};
            let specular_map = if (mat.specular_texture != "") {TextureDesc::from_file(ctx, base_dir.join(std::path::Path::new(&mat.specular_texture).to_path_buf()))} else {None};


            let mat_desc = MaterialDesc {
                ambient_color: if !mat.ambient.iter().all(|v| *v == 0.0) {Vec3::from_array(mat.ambient)} else {Default::default()},
                diffuse_color: if !mat.diffuse.iter().all(|v| *v == 0.0) {Vec3::from_array(mat.diffuse)} else {Default::default()},
                specular_color: if !mat.specular.iter().all(|v| *v == 0.0) {Vec3::from_array(mat.specular)} else {Default::default()},
                transparancy: mat.dissolve,
                shininess: mat.shininess,
                lighting_type: lighting_type,
                ambient_map,
                diffuse_map,
                specular_map,
            };

            mat_descs.insert(mat_idx, mat_desc);
        }

        for model in models.iter() {
            let mesh = &model.mesh;
            let mut mat_desc = &MaterialDesc::default();
            if let Some(mat_key) = mesh.material_id {
                mat_desc = mat_descs.get(&mat_key).unwrap_or(mat_desc);
            }
            let mesh_desc = MeshDesc::new(ctx, &model.name, mesh, mat_desc);
            meshes.push(mesh_desc);
        }

        Self {
            name,
            meshes,
            materials,
        }
    }
}

#[derive(Clone)]
pub struct MeshDesc {
    pub name: String,
    pub mesh: Mesh,
    pub mesh_id: usize,
    pub element_count: i32,
    pub binding: Bindings,
    pub model_matrix: glam::Mat4,
    pub material_id: usize,
    pub mat_desc: MaterialDesc,

    pub show: bool,
    pub highlight: bool,
    pub bounding_sphere: BoundingSphere,
}

impl MeshDesc {
    pub fn new(ctx: &mut Context, name: &str, mesh: &Mesh, mat_desc: &MaterialDesc) -> Self {
        let positions: &[f32] = &mesh.positions;
        let normals: &[f32] = &mesh.normals;
        let texcoords: &[f32] = &mesh.texcoords;
        let bs = BoundingSphere::from_min_max(positions);

        let mut vertex_data: Vec<f32> = vec![];
        for i in (0..(positions.len() / 3)) {
            vertex_data.extend_from_slice(&[positions[i * 3], positions[i * 3 + 1], positions[i * 3 + 2]]);
            if !normals.is_empty() {
                vertex_data.extend_from_slice(&[normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]]);
            } else {
                vertex_data.extend_from_slice(&[0.0; 3]);
            } 
            if !texcoords.is_empty() {
                vertex_data.extend_from_slice(&[texcoords[i * 2], texcoords[i * 2 + 1]]);
            } else {
                vertex_data.extend_from_slice(&[0.0; 2]);
            }
        }

        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, vertex_data.as_slice());

        let indices: &[u32] = &mesh.indices;
        let element_count = indices.len() as i32;

        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, indices);

        let binding = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        Self {
            name: name.to_string(),
            mesh: mesh.clone(),
            mesh_id: get_id(),
            element_count,
            binding,
            model_matrix: glam::Mat4::IDENTITY,
            material_id: mesh.material_id.unwrap_or(0),
            mat_desc: mat_desc.clone(),

            show: true,
            highlight: false,
            bounding_sphere: bs,
        }
    }
}

#[derive(Clone)]
pub struct BoundingSphere {
    pub center: Vec3,
    pub radius: f32,
}

impl BoundingSphere {
    pub fn from_min_max(points: &[f32]) -> Self {
        let mut center: Vec3 = vec3(0.0, 0.0, 0.0);
        let rcp_size = 1.0 / points.len() as f32;
        points.chunks(3).for_each(|v| {
            center.x += v[0] * rcp_size;
            center.y += v[1] * rcp_size;
            center.z += v[2] * rcp_size;
        });

        let mut radius: f32 = 0.0;
        points
            .chunks(3)
            .for_each(|v| {
                let dist = vec3(v[0], v[1], v[2]).distance(center);
                radius = if dist > radius {dist} else {radius};
            }
        );

        Self {
            center,
            radius: radius,
        }
    }
}
