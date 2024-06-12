use cgmath::Transform;
use std::future::Future;
use std::iter;
use std::process::exit;
use std::sync::{Arc, Mutex};
use wgpu;
use wgpu::util::{DeviceExt, StagingBelt};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub mod color;
pub mod mesh;
pub mod object;
pub mod primitives;
pub mod vertex;

use crate::engine::camera::{Camera, CameraUniform};
use crate::engine::graphics_backend::mesh::Mesh;
use crate::engine::graphics_backend::vertex::Vertex;
use crate::engine::ui::{UIRenderer, UIElement, text::{Text, TextOrigin, TextRenderer}};


pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    // pub window: Arc<Mutex<Window>>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    meshes: Vec<Mesh>,
    pub bg: [f32; 3],
    pub staging_belt: StagingBelt, // Add this line to include the staging belt
    pub ui_handler: UIRenderer
}

pub trait Backend {
    async fn new(window: &Window) -> Self;
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    fn update(&mut self, new_mesh_data: Vec<(Vec<Vertex>, Vec<u16>)>, bg: [f32; 3]);
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}

impl Backend for State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let surface = unsafe { instance.create_surface(window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: adapter.features(),
                    limits: adapter.limits(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let aspect_ratio = size.width as f32 / size.height as f32;
        let camera = Camera::new(cgmath::Point3::new(0.0, 0.0, 5.0), aspect_ratio);
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        });

        let ui_handler = UIRenderer::new(
            &device,
            &config,
            "./src/engine/graphics_backend/Inter-Medium.ttf",
        );

        let staging_belt = StagingBelt::new(1024);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            camera,
            camera_buffer,
            camera_uniform,
            camera_bind_group,
            // window,
            meshes: Vec::new(),
            bg: [0.0, 0.0, 0.0],
            staging_belt,
            ui_handler
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.camera.aspect_ratio = new_size.width as f32 / new_size.height as f32;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn update(&mut self, new_mesh_data: Vec<(Vec<Vertex>, Vec<u16>)>, bg: [f32; 3]) {
        self.camera_uniform.update(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        self.bg = bg;

        let view_projection = self.camera.projection_matrix() * self.camera.view_matrix();

        self.meshes = new_mesh_data
            .into_iter()
            .map(|(vertices, indices)| {
                let transformed_vertices: Vec<Vertex> = vertices
                    .iter()
                    .map(|vertex| {
                        let mut pos = cgmath::Point3::new(
                            vertex.position[0],
                            vertex.position[1],
                            vertex.position[2],
                        );
                        let transformed_pos = view_projection.transform_point(pos);
                        Vertex {
                            position: [transformed_pos.x, transformed_pos.y, transformed_pos.z],
                            ..*vertex
                        }
                    })
                    .collect();

                let vertex_buffer =
                    self.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Vertex Buffer"),
                            contents: bytemuck::cast_slice(&transformed_vertices),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                let index_buffer =
                    self.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Index Buffer"),
                            contents: bytemuck::cast_slice(&indices),
                            usage: wgpu::BufferUsages::INDEX,
                        });

                Mesh {
                    vertex_buffer,
                    index_buffer,
                    num_indices: indices.len() as u32,
                }
            })
            .collect();

        // Example usage of text renderer
        // let example_text = Text {
        //     content: String::from("Hello World"),
        //     position: cgmath::Point2::new(0.0, 0.0),
        //     color: [0.0, 0.0, 1.0, 1.0],
        //     origin: TextOrigin::Center,
        // };
        // self.ui_handler.queue(UIElement::Text(example_text)); // Add the text to the texts vector
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.bg[0] as f64,
                            g: self.bg[1] as f64,
                            b: self.bg[2] as f64,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Render the cubes
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            for mesh in &self.meshes {
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
            }
        } // Drop the render_pass here to release the mutable borrow on encoder

        // Render the text
        self.ui_handler.draw(
            self.config.width as f32,
            self.config.height as f32,
        );

        self.ui_handler.render(
            &self.device,
            &mut encoder,
            &view,
            &mut self.staging_belt,
            self.config.width,
            self.config.height,
        );

        self.staging_belt.finish();

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        self.staging_belt.finish();
        self.staging_belt.recall();

        Ok(())
    }
}
