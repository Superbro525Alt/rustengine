use crate::engine::component;
use crate::engine::graphics_backend::{
    object::Object, primitives::Cube, vertex::Vertex, Backend, State,
};
// use crate::engine::state::CustomEvent;
use rand::Rng;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

use crate::engine::state::AppEvent;


pub struct Renderer {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub render_queue: Arc<Mutex<Vec<component::RenderOutput>>>,
    pub window: Arc<Mutex<Window>>,
    backend: State,
}

impl Renderer {
    pub async fn new(
        title: String,
        width: u32,
        height: u32,
        event_loop: &EventLoop<()>,
    ) -> (Self, WindowId) {
        let window = Arc::new(Mutex::new(
            WindowBuilder::new()
                .with_title(&title)
                .with_inner_size(winit::dpi::LogicalSize::new(width as f64, height as f64))
                .build(&event_loop)
                .expect("Failed to build window"),
        ));

        let backend = State::new(window.clone()).await;

        (Self {
            title,
            width,
            height,
            render_queue: Arc::new(Mutex::new(Vec::new())),
            window: window.clone(),
            backend,
        }, window.clone().lock().unwrap().id().clone())
    }

    pub fn run(renderer: Arc<Mutex<Self>>, rx: Receiver<AppEvent>, control_tx: Sender<ControlFlow>) {
    let mut rng = rand::thread_rng(); // Assuming used elsewhere
    let mut t = std::time::SystemTime::now();
    let mut times: Vec<std::time::SystemTime> = vec![];

    loop {
        match rx.recv() {
            Ok(event) => match event {
                AppEvent::KeyPressed(key_code) => {
                    if key_code == winit::event::VirtualKeyCode::Escape {
                        control_tx.send(ControlFlow::Exit).unwrap();
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
                        break;
                    }
                },
                AppEvent::Resized(physical_size) => {
                    renderer.lock().unwrap().backend.resize(physical_size);
                },
                AppEvent::Closed => {
                    control_tx.send(ControlFlow::Exit).unwrap();
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
                    break;
                },
                _ => { control_tx.send(ControlFlow::Poll); }
            },
            Err(_) => break,
        }
    }
}
}
