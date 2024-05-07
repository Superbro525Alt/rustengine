use bytemuck_derive::{Pod, Zeroable};
use cgmath;
use cgmath::Angle;
use cgmath::InnerSpace;
use cgmath::SquareMatrix;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        let view = camera.view_matrix();
        let proj = camera.projection_matrix();
        self.view_proj = (proj * view).into();
    }
}

pub struct Camera {
    pub position: cgmath::Point3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub up: cgmath::Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub rotation_angle: f32,
}

impl Camera {
    pub fn new(position: cgmath::Point3<f32>, aspect_ratio: f32) -> Self {
        Camera {
            position,
            yaw: -90.0, // Default facing along -Z
            pitch: 0.0,
            up: cgmath::Vector3::unit_y(),
            aspect_ratio,
            fov: 45.0,
            near: 0.1,
            far: 100.0,
            rotation_angle: 1.0,
        }
    }

    pub fn view_matrix(&self) -> cgmath::Matrix4<f32> {
        let front = cgmath::Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize();
        cgmath::Matrix4::look_at_rh(self.position, self.position + front, self.up)
    }

    pub fn projection_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::perspective(
            cgmath::Deg(self.fov),
            self.aspect_ratio,
            self.near,
            self.far,
        )
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        let sensitivity = 0.1; // Sensitivity factor, adjust as needed

        // Update yaw and pitch with sensitivity adjustment
        self.yaw += delta_x * sensitivity;
        self.pitch += delta_y * sensitivity;

        // Clamp pitch to prevent flipping over at the poles
        self.pitch = self.pitch.clamp(-89.0, 89.0);

        // Update forward vector based on new yaw and pitch
        // self.update_camera_vectors();
    }
}
