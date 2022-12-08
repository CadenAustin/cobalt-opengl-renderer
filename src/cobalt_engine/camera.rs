use gltf::json::scene::UnitQuaternion;
use na::{Vector3, Rotation3};
use nalgebra as na;

pub enum CameraType {
    PERSPECTIVE,
    ORTHOGRAPHIC,
}

pub trait Camera {
    fn generate_view_matrix(&mut self);
    fn generate_projection_matrix(&mut self);
    fn update_matrices(&mut self);
    fn get_view_projection(&self) -> na::Matrix4<f32>;
    fn translate(&mut self, translation_vector: Vector3<f32>);
    fn rotate_by_quat(&mut self, rotation_quat: UnitQuaternion);
}

pub struct PerspectiveCamera {
    pub name: String,
    pub projection_matrix: na::Perspective3<f32>,
    pub view_matrix: na::Matrix4<f32>,
    pub aspect_ratio: f32,
    pub y_fov: f32,
    pub z_far: f32,
    pub z_near: f32,
    pub eye: na::Point3<f32>,
    pub front: na::Vector3<f32>
}

impl Camera for PerspectiveCamera {
    fn generate_view_matrix(&mut self) {
        self.projection_matrix =
            na::Perspective3::new(self.aspect_ratio, self.y_fov, self.z_near, self.z_far);
    }

    fn generate_projection_matrix(&mut self) {
        self.view_matrix = na::Matrix4::look_at_rh(&self.eye, &(self.eye + self.front), &na::Vector3::y());
    }

    fn update_matrices(&mut self) {
        self.generate_view_matrix();
        self.generate_projection_matrix();
        println!("{:?}", self.get_view_projection())
    }

    fn get_view_projection(&self) -> na::Matrix4<f32> {
        self.view_matrix * self.projection_matrix.as_matrix()
    }

    fn translate(&mut self, translation_vector: Vector3<f32>) {
        self.eye += translation_vector;
        self.update_matrices();
    }

    fn rotate_by_quat(&mut self, rotation_quat: UnitQuaternion) {
    }
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        Self {
            name: Default::default(),
            projection_matrix: na::Perspective3::new(1.0, 45.0, 1.0, 100.0),
            view_matrix: Default::default(),
            aspect_ratio: Default::default(),
            y_fov: Default::default(),
            z_far: Default::default(),
            z_near: Default::default(),
            eye: Default::default(),
            front: Default::default(),
        }
    }
}

pub struct OrthographicCamera {
    pub name: String,
    pub projection_matrix: na::Orthographic3<f32>,
    pub view_matrix: na::Matrix4<f32>,
    pub x_mag: f32,
    pub y_mag: f32,
    pub z_far: f32,
    pub z_near: f32,
    pub eye: na::Point3<f32>,
    pub front: na::Vector3<f32>
}

impl Camera for OrthographicCamera {
    fn generate_view_matrix(&mut self) {
        self.projection_matrix =
            na::Orthographic3::new(0.0, self.x_mag, 0.0, self.y_mag, self.z_near, self.z_far);
    }

    fn generate_projection_matrix(&mut self) {
        self.view_matrix = na::Matrix4::look_at_rh(&self.eye, &(self.eye + self.front), &na::Vector3::y());
    }

    fn update_matrices(&mut self) {
        self.generate_view_matrix();
        self.generate_projection_matrix();
    }

    fn get_view_projection(&self) -> na::Matrix4<f32> {
        self.view_matrix * self.projection_matrix.as_matrix()
    }

    fn translate(&mut self, translation_vector: Vector3<f32>) {
        self.eye += translation_vector;
        self.update_matrices();
    }

    fn rotate_by_quat(&mut self, rotation_quat: UnitQuaternion) {
    }
}

impl Default for OrthographicCamera {
    fn default() -> Self {
        Self {
            name: Default::default(),
            projection_matrix: na::Orthographic3::new(1.0, 10.0, 2.0, 20.0, 0.1, 1000.0),
            view_matrix: Default::default(),
            x_mag: Default::default(),
            y_mag: Default::default(),
            z_far: Default::default(),
            z_near: Default::default(),
            eye: Default::default(),
            front: Default::default(),
        }
    }
}

pub struct CameraBuilder {
    pub camera_type: CameraType,
    pub name: String,
    pub aspect_ratio: f32,
    pub x_mag: f32,
    pub y_mag: f32,
    pub y_fov: f32,
    pub z_far: f32,
    pub z_near: f32,
    pub eye: na::Point3<f32>,
}

impl CameraBuilder {
    pub fn new(camera_type: CameraType) -> Self {
        Self {
            camera_type,
            ..Default::default()
        }
    }

    pub fn name(&mut self, value: String) {
        self.name = value;
    }

    pub fn aspect_ratio(&mut self, value: f32) {
        self.aspect_ratio = value;
    }

    pub fn x_mag(&mut self, value: f32) {
        self.x_mag = value;
    }

    pub fn y_mag(&mut self, value: f32) {
        self.y_mag = value;
    }

    pub fn y_fov(&mut self, value: f32) {
        self.y_fov = value;
    }

    pub fn z_far(&mut self, value: f32) {
        self.z_far = value;
    }

    pub fn z_near(&mut self, value: f32) {
        self.z_near = value;
    }

    pub fn eye(&mut self, value: na::Point3<f32>) {
        self.eye = value;
    }

    pub fn build(self) -> Box<dyn Camera> {
        let mut camera: Box<dyn Camera> = match self.camera_type {
            CameraType::PERSPECTIVE => Box::new(PerspectiveCamera {
                name: self.name,
                aspect_ratio: self.aspect_ratio,
                y_fov: self.y_fov,
                z_far: self.z_far,
                z_near: self.z_near,
                eye: self.eye,
                front: na::Vector3::new(0.0, 0.0, -1.0),
                ..Default::default()
            }),
            CameraType::ORTHOGRAPHIC => Box::new(OrthographicCamera {
                name: self.name,
                x_mag: self.x_mag,
                y_mag: self.y_mag,
                z_far: self.z_far,
                z_near: self.z_near,
                eye: self.eye,
                front: na::Vector3::new(0.0, 0.0, 1.0),
                ..Default::default()
            }),
        };

        camera.update_matrices();
        camera
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            camera_type: CameraType::PERSPECTIVE,
            name: Default::default(),
            aspect_ratio: 1.0,
            x_mag: Default::default(),
            y_mag: Default::default(),
            y_fov: 0.1,
            z_far: 100.0,
            z_near: Default::default(),
            eye: Default::default(),
        }
    }
}

pub fn camera_builder(camera_type: CameraType) -> CameraBuilder {
    CameraBuilder::new(camera_type)
}
