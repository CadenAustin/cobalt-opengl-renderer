use std::{
    f32::consts::PI,
    ops::Add,
    path::{Path, PathBuf}, time::Instant
};

use egui::DroppedFile;
use glam::{vec3, Mat4};
use miniquad::{
    BufferLayout, Context, EventHandler, Pipeline,
    PipelineParams, Shader, ShaderMeta,
    UniformBlockLayout, UniformDesc, VertexAttribute, VertexFormat, RenderPass, TextureFormat, TextureParams, Texture, PassAction, Bindings, Buffer, BufferType, UniformType,
};
use tobj;

use crate::{
    camera::Camera,
    egui_menu::draw_egui,
    scene::{MeshDesc, ObjDesc},
    shader::ShaderDesc,
};

const PAN_SPEED: f32 = 0.2;
const ZOOM_SPEED: f32 = 0.002;

#[repr(C)]
pub struct RenderUniforms {
    pub view_proj: glam::Mat4,
    pub model_matrix: glam::Mat4,
    pub camera_pos: [f32; 3],
    pub light_pos: [f32; 3],
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub ambient_map_exists: u32,
    pub ambient_map: u32,
    pub diffuse_map_exists: u32,
    pub diffuse_map: u32,
    pub specular_map_exists: u32,
    pub specular_map: u32,
    pub shininess: f32,
    pub highlight: u32,
}

#[repr(C)]
pub struct PostProcessingUniforms {
    pub resolution: glam::Vec2,
    pub offset: f32,
}

pub struct Stage {
    pub egui_mq: egui_miniquad::EguiMq,
    pub egui_input: egui::RawInput,
    render_pipeline: Pipeline,
    render_pass: RenderPass,
    post_processing_pipeline: Pipeline,
    post_processing_bind: Bindings,
    pub objs: Vec<ObjDesc>,
    camera: Camera,
    view_proj: Mat4,
    width: f32,
    height: f32,

    mouse_pan: bool,
    mouse_orbit: bool,
    last_x: f32,
    last_y: f32,
    time: Instant
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
        let (w, h) = ctx.screen_size();
        let color_img = Texture::new_render_texture(
            ctx,
            TextureParams {
                width: w as _,
                height: h as _,
                format: TextureFormat::RGBA8,
                ..Default::default()
            },
        );
        let depth_img = Texture::new_render_texture(
            ctx,
            TextureParams {
                width: w as _,
                height: h as _,
                format: TextureFormat::Depth,
                ..Default::default()
            },
        );

        let render_pass = RenderPass::new(ctx, color_img, depth_img);

        let objs = vec![ObjDesc::from_obj_file(
            ctx,
            Path::new("models/gto67.obj").to_path_buf(),
        )];

        let meta = ShaderMeta {
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("view_proj", miniquad::UniformType::Mat4),
                    UniformDesc::new("model_matrix", miniquad::UniformType::Mat4),
                    UniformDesc::new("light_pos", miniquad::UniformType::Float3),
                    UniformDesc::new("camera_pos", miniquad::UniformType::Float3),
                    UniformDesc::new("ambient_color", miniquad::UniformType::Float3),
                    UniformDesc::new("diffuse_color", miniquad::UniformType::Float3),
                    UniformDesc::new("specular_color", miniquad::UniformType::Float3),
                    UniformDesc::new("ambient_map_exists", miniquad::UniformType::Int1),
                    UniformDesc::new("ambient_map", miniquad::UniformType::Int1),
                    UniformDesc::new("diffuse_map_exists", miniquad::UniformType::Int1),
                    UniformDesc::new("diffuse_map", miniquad::UniformType::Int1),
                    UniformDesc::new("specular_map_exists", miniquad::UniformType::Int1),
                    UniformDesc::new("specular_map", miniquad::UniformType::Int1),
                    UniformDesc::new("shininess", miniquad::UniformType::Float1),
                    UniformDesc::new("highlight", miniquad::UniformType::Int1),
                ],
            },
            images: vec![],
        };

        let render_shader_desc =
            ShaderDesc::from_file("shaders/shader.vs", "shaders/shader.fs", meta.clone()).unwrap();

        let render_shader =
            Shader::new(ctx, &render_shader_desc.vert, &render_shader_desc.frag, render_shader_desc.meta).unwrap();

        let render_pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout {
                ..Default::default()
            }],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("normal", VertexFormat::Float3),
                VertexAttribute::new("texcoord", VertexFormat::Float2),
            ],
            render_shader,
            PipelineParams {
                depth_test: miniquad::Comparison::LessOrEqual,
                depth_write: true,

                ..Default::default()
            },
        );

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            /* pos         uvs */
            -1.0, -1.0,    0.0, 0.0,
             1.0, -1.0,    1.0, 0.0,
             1.0,  1.0,    1.0, 1.0,
            -1.0,  1.0,    0.0, 1.0,
        ];

        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: &[u16] = &[0, 1, 2, 0, 2, 3];

        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let post_processing_bind = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![color_img],
        };
    

        /*
        let post_shader_desc =
            ShaderDesc::from_file("shaders/blur_post.vs", "shaders/blur_post.fs", ShaderMeta {
                images: vec!["tex".to_string()],
                uniforms: UniformBlockLayout {
                    uniforms: vec![UniformDesc::new("resolution", UniformType::Float2)],
                },
            }).unwrap();
            */

            let post_shader_desc =
            ShaderDesc::from_file("shaders/wave_post.vs", "shaders/wave_post.fs", ShaderMeta {
                images: vec!["tex".to_string()],
                uniforms: UniformBlockLayout {
                    uniforms: vec![UniformDesc::new("resolution", UniformType::Float2), UniformDesc::new("offset", UniformType::Float1)],
                },
            }).unwrap();

        let post_shader =
            Shader::new(ctx, &post_shader_desc.vert, &post_shader_desc.frag, post_shader_desc.meta).unwrap();

        let post_pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            post_shader,
        );

        let (width, height) = ctx.screen_size();
        let mut camera = Camera::default();
        camera.orbital();
        let view_proj = camera.look_at_view_proj(width, height);

        Self {
            egui_mq: egui_miniquad::EguiMq::new(ctx),
            egui_input: Default::default(),
            render_pipeline,
            render_pass,
            post_processing_pipeline: post_pipeline,
            post_processing_bind,
            objs,
            camera,
            view_proj,
            width,
            height,

            mouse_pan: false,
            mouse_orbit: false,
            last_x: 0.0,
            last_y: 0.0,
            time: Instant::now()
        }
    }

    pub fn update_camera(&mut self) {
        self.camera.orbital();
        self.view_proj = self.camera.look_at_view_proj(self.width, self.height)
    }

    pub fn reset_camera(&mut self) {
        self.camera = Camera::default();
        self.camera.orbital();
        self.view_proj = self.camera.look_at_view_proj(self.width, self.height);
    }

    pub fn set_camera_target(&mut self, mesh: &MeshDesc) {
        self.camera.look_at_center = mesh.bounding_sphere.center;
        self.camera.radius = mesh.bounding_sphere.radius;
        self.update_camera();
    }

    pub fn add_new_object(&mut self, ctx: &mut Context, file_path: PathBuf) {
        let obj = ObjDesc::from_obj_file(ctx, file_path);
        self.set_camera_target(&obj.meshes[0]);
        for mesh in &obj.meshes {
            if obj.materials.len() > mesh.material_id {
                println!("{:?}", obj.materials[mesh.material_id]);
            }
            println!("{}: {:?}", mesh.name, mesh.mesh.texcoords)
        }
        self.objs.push(obj);
        self.update_camera();
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut miniquad::Context) {
        
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let color_img = Texture::new_render_texture(
            ctx,
            TextureParams {
                width: width as _,
                height: height as _,
                format: TextureFormat::RGBA8,
                ..Default::default()
            },
        );
        let depth_img = Texture::new_render_texture(
            ctx,
            TextureParams {
                width: width as _,
                height: height as _,
                format: TextureFormat::Depth,
                ..Default::default()
            },
        );

        let offscreen_pass = RenderPass::new(ctx, color_img, depth_img);

        self.render_pass.delete(ctx);
        self.render_pass = offscreen_pass;
        self.post_processing_bind.images[0] = color_img;
    }

    fn draw(&mut self, ctx: &mut miniquad::Context) {
        ctx.begin_pass(
            self.render_pass,
            PassAction::clear_color(1.0, 1.0, 1.0, 1.0),
        );
        ctx.apply_pipeline(&self.render_pipeline);

        for obj in &self.objs {
            for mesh in &obj.meshes {
                if (mesh.show == false) {
                    continue;
                };
                ctx.apply_bindings(&mesh.binding);

                let mut ambient_map_exists = 0;
                let ambient_map_id = if let Some(texture) = mesh.mat_desc.ambient_map {
                    ambient_map_exists = 1;
                    texture.gl_id
                } else {
                    0
                };

                let mut diffuse_map_exists = 0;
                let diffuse_map_id = if let Some(texture) = mesh.mat_desc.diffuse_map {
                    diffuse_map_exists = 1;
                    texture.gl_id
                } else {
                    0
                };

                let mut specular_map_exists = 0;
                let specular_map_id = if let Some(texture) = mesh.mat_desc.specular_map {
                    specular_map_exists = 1;
                    texture.gl_id
                } else {
                    0
                };

                let uniforms = RenderUniforms {
                    view_proj: self.view_proj,
                    model_matrix: mesh.model_matrix,
                    light_pos: self.camera.pos.to_array(),
                    camera_pos: self.camera.pos.to_array(),
                    ambient_color: mesh.mat_desc.diffuse_color.to_array(),
                    diffuse_color: mesh.mat_desc.diffuse_color.to_array(),
                    specular_color: mesh.mat_desc.diffuse_color.to_array(),
                    ambient_map_exists,
                    ambient_map: ambient_map_id,
                    diffuse_map_exists,
                    diffuse_map: diffuse_map_id,
                    specular_map_exists,
                    specular_map: specular_map_id,
                    shininess: mesh.mat_desc.shininess,
                    highlight: if mesh.highlight { 1 } else { 0 },
                };

                ctx.apply_uniforms(&uniforms);
                ctx.draw(0, mesh.element_count, 1);
            }
        }
        ctx.end_render_pass();

        ctx.begin_default_pass(PassAction::Nothing);
        ctx.apply_pipeline(&self.post_processing_pipeline);
        ctx.apply_bindings(&self.post_processing_bind);
        let (w, h) = ctx.screen_size();
        ctx.apply_uniforms(&PostProcessingUniforms {
            resolution: glam::vec2(w, h),
            offset: self.time.elapsed().as_millis() as f32 / 1000.0 * 2.0 * PI * 0.75,
        });
        ctx.draw(0, 6, 1);
        ctx.end_render_pass();

        draw_egui(self, ctx);

        ctx.commit_frame();
    }

    fn char_event(
        &mut self,
        _ctx: &mut Context,
        character: char,
        _keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.char_event(character);

        match character {
            'w' => {
                self.camera.move_forward(-0.2);
            }
            's' => {
                self.camera.move_backward(-0.2);
            }
            'a' => {
                self.camera.strafe_left(0.2);
            }
            'd' => {
                self.camera.strafe_right(0.2);
            }
            'c' => self.reset_camera(),
            _ => {}
        };
        self.update_camera();
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.egui_mq.mouse_wheel_event(x, y);

        self.camera.radius = self.camera.radius.add(-y * ZOOM_SPEED);
        self.update_camera();
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);

        if self.last_x == 0.0 && self.last_y == 0.0 {
            self.last_x = x;
            self.last_y = y;
        }

        let x_offset = x - self.last_x;
        let y_offset = y - self.last_y;
        self.last_x = x;
        self.last_y = y;

        if self.mouse_pan {
            let offset = PAN_SPEED * vec3(-x_offset, y_offset, 0.0);
            self.camera.look_at_center += offset;
            self.update_camera();
        }

        if self.mouse_orbit {
            let x_angle = x_offset * (2. * PI) * 0.01;
            let y_angle = y_offset * (PI) * 0.01;

            self.camera.rotate_azimuth(x_angle.to_radians());
            self.camera.rotate_polar(y_angle.to_radians());
            self.update_camera();
        }
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: miniquad::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_down_event(ctx, button, x, y);
        match button {
            miniquad::MouseButton::Right => {
                self.mouse_pan = false;
                self.mouse_orbit = true;
            }
            miniquad::MouseButton::Left => {}
            miniquad::MouseButton::Middle => {
                self.mouse_pan = true;
                self.mouse_orbit = false;
            }
            miniquad::MouseButton::Unknown => {}
        }
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: miniquad::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_up_event(ctx, button, x, y);
        match button {
            miniquad::MouseButton::Right => self.mouse_orbit = false,
            miniquad::MouseButton::Left => {}
            miniquad::MouseButton::Middle => self.mouse_pan = false,
            miniquad::MouseButton::Unknown => {}
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut miniquad::Context,
        keycode: miniquad::KeyCode,
        keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut miniquad::Context,
        keycode: miniquad::KeyCode,
        keymods: miniquad::KeyMods,
    ) {
        self.egui_mq.key_up_event(keycode, keymods);
    }

    fn files_dropped_event(&mut self, ctx: &mut Context) {
        for idx in 0..ctx.dropped_file_count() {
            let file_path = ctx.dropped_file_path(idx);
            self.egui_input.dropped_files.push(DroppedFile {
                path: file_path.clone(),
                name: "".to_string(),
                last_modified: None,
                bytes: ctx.dropped_file_bytes(idx).map(|bytes| bytes.into()),
            });
            self.add_new_object(ctx, file_path.unwrap());
        }
    }
}
