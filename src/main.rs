use cobalt_engine::camera::{camera_builder, CameraType};
use cobalt_engine::mesh::Mesh;
use nalgebra as na;
use std::path::{Path, PathBuf};

use cobalt_engine::buffer::{VAO, VBO, EBO};
use cobalt_engine::program::{Program, Shader};
use cobalt_engine::scene::{generate_scenes_from_gltf, SceneDesc};
use cobalt_engine::triangle::{Vertex};

use renderdoc::{RenderDoc, V110};

extern crate gl;
extern crate sdl2;

pub mod cobalt_engine;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const CLEAR_COLOR: (f32, f32, f32) = (0.3, 0.3, 0.5);

fn main() {
    let rd_result = RenderDoc::new();
    match rd_result {
        Ok(_) => {
            let mut rd: RenderDoc<V110> = rd_result.unwrap();
            rd.set_focus_toggle_keys(&[renderdoc::InputButton::F]);
            rd.set_capture_keys(&[renderdoc::InputButton::C]);
            rd.set_capture_option_u32(renderdoc::CaptureOption::AllowVSync, 1);
            rd.set_capture_option_u32(renderdoc::CaptureOption::ApiValidation, 1);
        },
        Err(_) => {
            
        },
    }
    


    let sdl = sdl2::init().unwrap();
    let video_sub = sdl.video().unwrap();

    let gl_attr = video_sub.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_sub
        .window("Cobalt", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_ctx = window.gl_create_context().unwrap();

    let gl = gl::load_with(|s| video_sub.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

    let program = Program::from_shaders(vec![
        Shader::from_source(&PathBuf::from("shaders/simple.vs")).unwrap(),
        Shader::from_source(&PathBuf::from("shaders/simple.fs")).unwrap(),
    ])
    .unwrap();

    let mut scenes =
        generate_scenes_from_gltf(&PathBuf::from("scenes/Cameras/glTF/Cameras.gltf"));

    let mut active_scene = scenes.first_mut().unwrap();

    unsafe {
        let (clear_r, clear_g, clear_b) = CLEAR_COLOR;
        gl::ClearColor(clear_r, clear_g, clear_b, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'event_loop,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        program.set_used();

        program.insert_uniform(
            "proj_view",
            cobalt_engine::program::UniformEnum::Matrix4(active_scene.cameras[0].get_view_projection()),
        );

        for m in &active_scene.meshes {
            m.render(&program);
        }

        window.gl_swap_window();
    }
}
