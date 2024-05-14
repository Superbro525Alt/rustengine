use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget, EventLoopProxy},
    window::{Window, WindowBuilder},
};

use std::sync::{Arc, Mutex, mpsc::{self, Sender, Receiver}};
use crate::engine::graphics_backend::{object::Object, primitives::Cube, vertex::Vertex, Backend, State};
use rand::Rng;
use wgpu;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::engine::component::RenderOutput;

pub struct Renderer {
    pub title: String,
    pub width: u32,
    pub height: u32,
    window: Arc<Mutex<Window>>,
    backend: Arc<Mutex<Option<Box<State>>>>,
    pub active: bool,
    pub render_queue: Arc<Mutex<Vec<RenderOutput>>>,
    pub render_tx: Option<Sender<RenderOutput>>,
}

unsafe impl Send for Renderer {}
unsafe impl Sync for Renderer {}

impl Renderer {
    pub fn none() -> Self {
        Self {
            title: String::new(),
            width: 0,
            height: 0,
            window: Arc::new(Mutex::new(WindowBuilder::new().build(&EventLoop::new()).unwrap())),
            backend: Arc::new(Mutex::new(None)),
            active: false,
            render_queue: Arc::new(Mutex::new(vec![])),
            render_tx: None,
        }
    }

    pub async fn new(title: String, width: u32, height: u32) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new();
        let window = Arc::new(Mutex::new(
            WindowBuilder::new()
                .with_title(&title)
                .with_inner_size(winit::dpi::LogicalSize::new(width as f64, height as f64))
                .build(&event_loop)
                .expect("Failed to build window"),
        ));

        let backend = State::new(window.clone()).await;

        let (render_tx, render_rx) = mpsc::channel();

        let renderer = Self {
            title,
            width,
            height,
            window,
            backend: Arc::new(Mutex::new(Some(Box::new(backend)))),
            active: true,
            render_queue: Arc::new(Mutex::new(vec![])),
            render_tx: Some(render_tx),
        };

        renderer.start_render_thread(render_rx);

        (renderer, event_loop)
    }

    pub fn start_render_thread(&self, render_rx: Receiver<RenderOutput>) {
        let backend = self.backend.clone();
        let window = self.window.clone();

        std::thread::spawn(move || {
            let mut state = backend.lock().unwrap().take().expect("PANIC");
            let mut rng = rand::thread_rng();
            let mut t = SystemTime::now();
            let mut times: Vec<SystemTime> = vec![];

            loop {
                match render_rx.recv() {
                    Ok(mut render_output) => {
                        state.update(vec![render_output.raw_desc()], [0.0, 0.0, 1.0]);
                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => break,
                            Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }

    pub fn update(&mut self) {}

    pub fn run(renderer: Arc<Mutex<Self>>, event_loop: Arc<Mutex<EventLoop<()>>>) {
        let window = renderer.lock().unwrap().window.clone();
        let backend = renderer.lock().unwrap().backend.clone();
        // let eloop = event_loop.clone().lock().unwrap();

        event_loop.clone().lock().unwrap().run(move |event: Event<()>, _: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow| {
            match event {
                Event::WindowEvent { ref event, window_id } if window_id == window.lock().unwrap().id() => {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            backend.lock().unwrap().as_mut().unwrap().resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            backend.lock().unwrap().as_mut().unwrap().resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == window.lock().unwrap().id() => {
                    backend.lock().unwrap().as_mut().unwrap().render().unwrap();
                }
                Event::RedrawEventsCleared => {
                    window.lock().unwrap().request_redraw();
                }
                _ => {}
            }
        });
    }

    pub fn render(&self, data: RenderOutput) {
        if let Some(ref tx) = self.render_tx {
            tx.send(data).unwrap();
        }
    }
}
