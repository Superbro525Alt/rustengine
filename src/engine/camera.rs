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

    pub fn rotate_around_origin(&mut self) {
        let rad: cgmath::Rad<f32> = cgmath::Deg(self.rotation_angle).into();
        let cos_angle = rad.cos();
        let sin_angle = rad.sin();
        let new_x = self.position.x * cos_angle - self.position.z * sin_angle;
        let new_z = self.position.x * sin_angle + self.position.z * cos_angle;
        self.position.x = new_x;
        self.position.z = new_z;
    }
}
