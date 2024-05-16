use crate::engine::camera;
use crate::engine::component;
use crate::engine::gameobject;
use crate::engine::renderer;
use crate::engine::static_component;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use winit::window;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder, WindowId},
};

#[derive(Debug, Clone)]
pub enum AppEvent {
    KeyPressed(winit::event::VirtualKeyCode),
    MouseInput(winit::event::MouseButton),
    Resized(winit::dpi::PhysicalSize<u32>),
    ScaleFactorChanged(winit::dpi::PhysicalSize<u32>),
    RedrawRequested,
    RedrawEventsCleared,
    Closed,
}

impl AppEvent {
    fn from_event(event: &Event<'_, ()>, win_id: &WindowId) -> Option<Self> {
        match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                window_id,
            } if window_id == win_id => input.virtual_keycode.map(AppEvent::KeyPressed),
            Event::WindowEvent {
                event: WindowEvent::MouseInput { button, .. },
                window_id,
            } if window_id == win_id => Some(AppEvent::MouseInput(*button)),
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                window_id,
            } if window_id == win_id => Some(AppEvent::Resized(*size)),
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
                window_id,
            } if window_id == win_id => Some(AppEvent::ScaleFactorChanged(**new_inner_size)),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == win_id => Some(AppEvent::Closed),
            Event::RedrawRequested(window_id) if window_id == win_id => {
                Some(AppEvent::RedrawRequested)
            }
            Event::RedrawEventsCleared => Some(AppEvent::RedrawEventsCleared),
            _ => None,
        }
    }
}

pub struct EngineState {
    objects: Vec<i32>,
    static_components: Vec<Arc<Mutex<dyn static_component::StaticComponent>>>,
}

impl EngineState {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            static_components: vec![],
        }
    }

    pub fn objects(&self) -> &Vec<i32> {
        &self.objects
    }

    pub fn add_object(&mut self, obj: i32) {
        self.objects.push(obj);
    }

    pub fn add_static(&mut self, obj: Arc<Mutex<dyn static_component::StaticComponent>>) {
        self.static_components.push(obj);
    }
}

pub struct Engine {
    pub state: EngineState,
    pub renderer: Arc<Mutex<renderer::Renderer>>,
    pub graphics: bool,
    pub event_loop_proxy: EventLoopProxy<()>,
    render_handle: Option<JoinHandle<()>>,
    event_tx: Option<Sender<AppEvent>>,
    control_rx: Option<Receiver<ControlFlow>>,
    win_id: WindowId,
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}

impl Engine {
    pub async fn new(graphics: bool, event_loop: EventLoop<()>) -> (Self, EventLoop<()>) {
        let event_loop_proxy = event_loop.create_proxy();

        let (mut renderer_instance, window_id) =
            renderer::Renderer::new(String::from("Engine"), 800, 600, &event_loop).await;
        let renderer = Arc::new(Mutex::new(renderer_instance));

        let (event_tx, event_rx) = mpsc::channel::<AppEvent>();
        let (control_tx, control_rx) = mpsc::channel::<ControlFlow>();

        let engine = Self {
            state: EngineState::new(),
            renderer: renderer.clone(),
            graphics,
            event_loop_proxy,
            render_handle: None,
            event_tx: Some(event_tx),
            control_rx: Some(control_rx),
            win_id: window_id,
        };

        if graphics {
            let renderer_clone = renderer.clone();
            thread::spawn(move || {
                renderer::Renderer::run(renderer_clone, event_rx, control_tx);
            });
        }

        (engine, event_loop)
    }

    pub fn state(&self) -> &EngineState {
        &self.state
    }

    pub fn renderer(&self) -> Arc<Mutex<renderer::Renderer>> {
        self.renderer.clone()
    }

    pub fn render(&mut self, data: component::RenderOutput) -> usize {
        let mut renderer = self.renderer.lock().unwrap();
        let mut render_queue = renderer.render_queue.lock().unwrap();
        render_queue.push(data);
        render_queue.len() - 1
    }

    pub fn remove_from_render_queue(&mut self, reference: usize) {
        let mut renderer = self.renderer.lock().unwrap();
        let mut render_queue = renderer.render_queue.lock().unwrap();
        render_queue.remove(reference);
    }

    pub fn input_data(&self) -> component::InputData {
        component::InputData {}
    }

    pub fn tick(&mut self) {
        for obj in self.state.objects.clone().iter() {
            gameobject::to_object(*obj, |game_object| {
                if game_object.state.parent_id.is_none() {
                    game_object.tick_all(self);
                }
            });
        }

        for comp in self.state.static_components.iter_mut() {
            comp.lock().unwrap().tick();
        }
    }

    pub fn add_object(&mut self, obj: gameobject::MutexdGameObject) -> i32 {
        let id = obj.clone().lock().unwrap().id();
        self.state.add_object(id);
        id
    }

    pub fn add_static(&mut self, comp: Arc<Mutex<dyn static_component::StaticComponent>>) {
        self.state.add_static(comp);
    }

    pub fn run(engine: Arc<Mutex<Self>>, event_loop: EventLoop<()>) {
        // println!("running");
    let self_clone = engine.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(16)); // Adjust timing as needed, e.g., ~60 Hz
            let mut engine = self_clone.lock().unwrap();
            engine.tick(); // Perform periodic update
        }
    });

    let event_tx = engine.lock().unwrap().event_tx.take().unwrap();
    let control_rx = engine.lock().unwrap().control_rx.take().unwrap();

    let win_id = engine.lock().unwrap().win_id;

    event_loop.run(move |event, _, control_flow| {
        if let Some(app_event) = AppEvent::from_event(&event, &win_id) {
            if event_tx.send(app_event).is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        match control_rx.try_recv() {
            Ok(new_control_flow) => *control_flow = new_control_flow,
            Err(_) => *control_flow = ControlFlow::Wait,
        }
    });
}
}
