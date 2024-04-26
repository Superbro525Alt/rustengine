use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use std::sync::{Arc, Mutex};

use crate::engine::graphics_backend::{State, Backend};
use wgpu;

pub struct Renderer {
    pub title: String,
    pub width: u32,
    pub height: u32,
    event_loop: Option<EventLoop<()>>,
    window: Box<Window>,
    backend: State,
}

impl Renderer {
    pub async fn new(title: String, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let window = Box::new(WindowBuilder::new()
            .with_title(&title)
            .with_inner_size(winit::dpi::LogicalSize::new(width as f64, height as f64))
            .build(&event_loop)
            .expect("Failed to build window"));

        Self {
            title,
            width,
            height,
            event_loop: Some(event_loop),
            window: window,
            backend: State::new(window).await,
        }
    }

    pub fn run(mut self) {
        let event_loop = self.event_loop.take().expect("EventLoop already taken");
        let state = self.backend;

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
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
                Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                    state.update();
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
                    state.window().request_redraw();
                }
                _ => {}
            }
        });
    }
}
