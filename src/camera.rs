use std::f32::consts::PI;

use glam::{Vec3, vec3, Mat4};

pub struct Camera {
    pub pos: Vec3,
    pub look_at_center: Vec3,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub azimuth: f32,
    pub polar: f32,
    pub radius: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self { 
            pos: vec3(0.0, 1.5, 3.0), 
            look_at_center: vec3(0.0, 0.0, 0.0),
            fov: 60.0, 
            near: 0.01, 
            far: 2500.0,
            azimuth: 0.0,
            polar: PI / 4.0,
            radius: 2.0,
        }
    }
}

impl Camera {
    pub fn look_at_view_proj(&self, width: f32, height: f32) -> Mat4 {
        let proj = Mat4::perspective_rh_gl(self.fov.to_radians(), width / height, self.near, self.far);
        let view = self.view_matrix();
        proj * view
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.pos,
            self.look_at_center,
            vec3(0.0, 1.0, 0.0),
        )
    }

    pub fn rotate_polar(&mut self, radians: f32) {
        self.polar += radians;

        let polar_cap = PI / 2.0 - 0.001;
        if (self.polar > polar_cap) {
            self.polar = polar_cap;
        }

        if (self.polar < -polar_cap) {
            self.polar = polar_cap;
        }
    }

    pub fn rotate_azimuth(&mut self, radians: f32) {
        self.azimuth += radians;

        self.azimuth = self.azimuth % (2.0 * PI);
        if (self.azimuth < 0.0) {
            self.azimuth += 2.0 * PI;
        }
    }

    pub fn orbital(&mut self) {
        let x = self.look_at_center.x + self.radius * self.polar.cos() * self.azimuth.cos();
        let y = self.look_at_center.y + self.radius * self.polar.sin();
        let z = self.look_at_center.z + self.radius * self.polar.cos() * self.azimuth.sin();
        self.pos = vec3(x, y, z);
    }

    pub fn move_forward(&mut self, dist: f32) {
        let view_dir_vec4 = self.view_matrix().row(2);
        let view_dir = vec3(view_dir_vec4.x, view_dir_vec4.y, view_dir_vec4.z);
        self.look_at_center += view_dir * dist * self.radius;
    }

    pub fn move_backward(&mut self, dist: f32) {
        self.move_forward(-dist);
    }

    pub fn strafe_left(&mut self, dist: f32) {
        let view_dir_vec4 = self.view_matrix().row(2);
        let view_dir = vec3(view_dir_vec4.x, view_dir_vec4.y, view_dir_vec4.z).normalize();
        let right_dir = view_dir.cross(vec3(0.0, 1.0, 0.0)).normalize();
        self.look_at_center += right_dir * dist * self.radius;
    }

    pub fn strafe_right(&mut self, dist: f32) {
        self.strafe_left(-dist);
    }
}

