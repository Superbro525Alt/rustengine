use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use std::sync::{Arc, Mutex};

use crate::engine::graphics_backend::{
    object::Object, primitives::Cube, vertex::Vertex, Backend, State,
};
use rand::Rng;
use wgpu;

pub struct Renderer {
    pub title: String,
    pub width: u32,
    pub height: u32,
    event_loop: Option<EventLoop<()>>,
    window: Option<Arc<Mutex<Window>>>,
    backend: Option<Box<State>>,
    pub active: bool,
}

impl Renderer {
    pub fn none() -> Self {
        Self {
            title: String::from(""),
            width: 0,
            height: 0,
            event_loop: None,
            window: None,
            backend: None,
            active: false,
        }
    }

    pub async fn new(title: String, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let window = Arc::new(Mutex::new(
            WindowBuilder::new()
                .with_title(&title)
                .with_inner_size(winit::dpi::LogicalSize::new(width as f64, height as f64))
                .build(&event_loop)
                .expect("Failed to build window"),
        ));

        let backend = State::new(window.clone()).await;

        Self {
            title,
            width,
            height,
            event_loop: Some(event_loop),
            window: Some(window),
            backend: Some(Box::new(backend)),
            active: true,
        }
    }

    pub fn run(mut self) {
        let event_loop = self.event_loop.take().expect("EventLoop already taken");
        let mut state = self.backend.expect("PANIC");
        let mut window = self.window.expect("PANIC");
        let mut rng = rand::thread_rng();
        let mut t = std::time::SystemTime::now();
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.lock().unwrap().id() => {
                    if !state.input(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_size is &&mut so w have to dereference it twice
                                state.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == window.lock().unwrap().id() => {
                    println!("{:?}", t.elapsed());
                    t = std::time::SystemTime::now();
                    state.update(
                        vec![
                            // ( [Vertex{position: [0.0, 0.0, 0.0], color: [1.0, 0.0, 0.0]}, Vertex{position: [1.0, 0.0, 0.0], color: [0.0, 1.0, 0.0]}, Vertex{position: [0.5, 1.0, 0.0], color: [0.0, 0.0, 1.0]}].to_vec(),
                            // [0, 1, 2, 0].to_vec())
                            Cube::new(0.5, [1.0, 0.0, 0.0]).desc_raw(),
                        ],
                        [
                            // rng.gen_range::<f32, _>(0.0..1.0),
                            // rng.gen_range::<f32, _>(0.0..1.0),
                            // rng.gen_range::<f32, _>(0.0..1.0),
                            0.0, 0.0, 1.0,
                        ],
                    );
                    match state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            state.resize(state.size)
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    }
                }
                Event::RedrawEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    window.lock().unwrap().request_redraw();
                }
                _ => {}
            }
        });
    }
}
