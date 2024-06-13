use crate::engine::component;
use crate::engine::graphics_backend::primitives;
use crate::engine::graphics_backend::{
    object::Object, primitives::Cube, vertex::Vertex, Backend, State,
};
use log::{info, warn, error};
// use crate::engine::state::CustomEvent;
use rand::Rng;
use std::ops::Index;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

use crate::engine::state::AppEvent;

use super::state::FrameData;

pub struct Renderer {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub render_queue: Arc<Mutex<Vec<component::RenderOutput>>>,
    pub window: Option<Window>,
    pub backend: State,
    pub dt: Option<Duration>,
}

unsafe impl Send for Renderer {}
unsafe impl Sync for Renderer {}

impl Renderer {
    pub async fn new(
        title: String,
        width: u32,
        height: u32,
        event_loop: &EventLoop<()>,
    ) -> (Self, WindowId) {
        info!("Creating window...");
        let window = match WindowBuilder::new()
                .with_title(&title)
                .with_inner_size(winit::dpi::LogicalSize::new(width as f64, height as f64))
                .build(&event_loop) {
            Ok(window) => {
                info!("Window created successfully.");
                window
            },
            Err(e) => {
                error!("Failed to create window: {:?}", e);
                panic!("Failed to create window");
            }
        };
        
        info!("Creating backend...");

        let backend = State::new(&window).await;
        let id = window.id().clone();

        info!("Window created!");

        (
            Self {
                title,
                width,
                height,
                render_queue: Arc::new(Mutex::new(Vec::new())),
                window: Some(window),
                backend,
                dt: None,
            },
            id
        )
    }

    pub fn stop(&mut self) {
        warn!("Dropping window...");
        drop(self.window.as_mut());
        self.window = None;
        // self.window = None;
    }

    pub fn run(
        renderer: Arc<Mutex<Self>>,
        rx: Receiver<AppEvent>,
        control_tx: Sender<ControlFlow>,
        frame_data_tx: Sender<FrameData>,
    ) {
        let mut rng = rand::thread_rng(); // Assuming used elsewhere
        let mut t = std::time::SystemTime::now();
        let mut times: Vec<std::time::SystemTime> = vec![];

        loop {
            let dt = renderer.lock().unwrap().dt;
            frame_data_tx.send(FrameData { dt });
            match rx.recv() {
                Ok(event) => match event {
                    AppEvent::KeyPressed(key_code) => {
                        if key_code == winit::event::VirtualKeyCode::Escape {
                            // let average_frame_time = times
                            //     .windows(2)
                            //     .filter_map(|w| w[1].duration_since(w[0]).ok())
                            //     .map(|duration| duration.as_millis())
                            //     .sum::<u128>()
                            //     as f64
                            //     / (times.len() - 1) as f64;
                            // println!(
                            //     "Average frame time: {} ms, FPS: {}",
                            //     average_frame_time,
                            //     1000.0 / average_frame_time
                            // );
                            // control_tx.send(ControlFlow::Exit);
                            // break;
                        }
                    }
                    AppEvent::Resized(physical_size) => {
                        renderer.lock().unwrap().backend.resize(physical_size);
                    }
                    AppEvent::ScaleFactorChanged(new_inner_size) => {
                        renderer.lock().unwrap().backend.resize(new_inner_size)
                    }
                    AppEvent::RedrawRequested => {
                        renderer.lock().unwrap().dt = Some(t.elapsed().unwrap());
                        // println!("Frame Time: {:?}", t.elapsed().unwrap());
                        t = std::time::SystemTime::now();
                        times.push(t);
                        let queue = renderer.lock().unwrap().render_queue.clone();

                        renderer.lock().unwrap().backend.update(
                            queue
                                .lock()
                                .unwrap()
                                .iter_mut()
                                .map(|out| {out.raw_desc()})
                                .collect(),
                            [0.0, 0.0, 0.0],
                        );

                        let render_result = renderer.lock().unwrap().backend.render();
                        match render_result {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                let size = renderer.lock().unwrap().backend.size;
                                renderer.lock().unwrap().backend.resize(size);
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                control_tx.send(ControlFlow::Exit);
                            }
                            Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                        }
                    }
                    AppEvent::RedrawEventsCleared => {
                        let mut lock = &mut renderer.lock().unwrap().window;
                        if lock.as_mut().is_none() { return; }
                        lock
                            .as_mut()
                            .unwrap()
                            // .lock()
                            // .unwrap()
                            .request_redraw();
                    }
                    AppEvent::Closed => {
                        // control_tx.send(ControlFlow::Exit);
                        info!("Closing...");
                        let average_frame_time = times
                            .windows(2)
                            .filter_map(|w| w[1].duration_since(w[0]).ok())
                            .map(|duration| duration.as_millis())
                            .sum::<u128>() as f64
                            / (times.len() - 1) as f64;
                        println!(
                            "Average frame time: {} ms, FPS: {}",
                            average_frame_time,
                            1000.0 / average_frame_time
                        );
                        // break;
                    }
                    _ => {
                        control_tx.send(ControlFlow::Poll);
                    }
                },
                Err(_) => break,
            }
        }
    }
}
