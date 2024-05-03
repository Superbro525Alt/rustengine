use bytemuck_derive::{Pod, Zeroable};
use wgpu;
use downcast_rs::impl_downcast;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub struct Vec2d {
    x: f32,
    y: f32
}

pub struct Vec3d {
    x: f32,
    y: f32,
    z: f32
}

impl Vec2d {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn to_v3(&mut self, mut z: Option<f32>) -> Vec3d {
        if z.is_none() {
            z = Some(0.0)
        }

        Vec3d::new(self.x, self.y, z.expect("NO Z VALUE FOR CONVERSION"))
    }
}

impl Vec3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn to_v2(&mut self) -> Vec2d {
        Vec2d::new(self.x, self.y)
    }
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
