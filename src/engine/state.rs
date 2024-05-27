use crate::engine::camera;
use crate::engine::component;
use crate::engine::gameobject;
use crate::engine::renderer;
use crate::engine::static_component;
use std::ops::Index;
use std::process::exit;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::SystemTime;
use std::time::{Duration, Instant};

use serde_json::to_string;
use winit::window;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder, WindowId},
};

use serde::{Deserialize, Serialize};

use crate::engine::physics::PhysicsEngine;

use super::save::EngineSaveData;
use super::static_component::StaticComponent;
use super::ui::UIElement;

#[derive(Debug, Clone)]
pub enum AppEvent {
    KeyPressed(winit::event::VirtualKeyCode),
    KeyReleased(winit::event::VirtualKeyCode),
    MouseInput(winit::event::MouseButton),
    MouseRelease(winit::event::MouseButton),
    MouseMoved((f64, f64)),
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
            } if window_id == win_id => match input.state {
                ElementState::Pressed => input.virtual_keycode.map(AppEvent::KeyPressed),
                ElementState::Released => input.virtual_keycode.map(AppEvent::KeyReleased),
            },
            Event::WindowEvent {
                event: WindowEvent::MouseInput { button, state, .. },
                window_id,
            } if window_id == win_id => match state {
                ElementState::Pressed => Some(AppEvent::MouseInput(*button)),
                ElementState::Released => Some(AppEvent::MouseRelease(*button)),
            },
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                window_id,
            } if window_id == win_id => Some(AppEvent::MouseMoved((position.x, position.y))),
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

    pub fn objects(&self) -> Vec<i32> {
        self.objects.clone()
    }

    pub fn add_object(&mut self, obj: i32) {
        self.objects.push(obj);
    }

    pub fn add_static(&mut self, obj: Arc<Mutex<dyn static_component::StaticComponent>>) {
        self.static_components.push(obj);
    }
}

pub struct FrameData {
    pub dt: Option<Duration>,
}

pub struct Engine {
    pub state: EngineState,
    pub renderer: Arc<Mutex<renderer::Renderer>>,
    pub graphics: bool,
    pub event_loop_proxy: EventLoopProxy<()>,
    render_handle: Option<JoinHandle<()>>,
    event_tx: Option<Sender<AppEvent>>,
    control_rx: Option<Receiver<ControlFlow>>,
    frame_data_rx: Option<Receiver<FrameData>>,
    win_id: WindowId,
    pub dt: Option<Duration>,
    keys_pressed: Vec<winit::event::VirtualKeyCode>,
    mouse_buttons_pressed: Vec<winit::event::MouseButton>,
    mouse_position: (f64, f64),
    pub physics_engine: PhysicsEngine,
    pub paused: bool
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
        let (frame_data_tx, frame_data_rx) = mpsc::channel::<FrameData>();

        let engine = Self {
            state: EngineState::new(),
            renderer: renderer.clone(),
            graphics,
            event_loop_proxy,
            render_handle: None,
            event_tx: Some(event_tx),
            control_rx: Some(control_rx),
            frame_data_rx: Some(frame_data_rx),
            win_id: window_id,
            dt: None,
            keys_pressed: Vec::new(),
            mouse_buttons_pressed: Vec::new(),
            mouse_position: (0.0, 0.0),
            physics_engine: PhysicsEngine::new(0.1),
            paused: false
        };

        if graphics {
            let renderer_clone = renderer.clone();
            thread::spawn(move || {
                renderer::Renderer::run(renderer_clone, event_rx, control_tx, frame_data_tx);
            });
        }

        (engine, event_loop)
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        self.paused = false;
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
        // if render_queue.len() == 2 {
        //     return 0;
        // }
        //
        // println!("pushed");
        render_queue.len() - 1
    }

    pub fn remove_from_render_queue(&mut self, reference: usize) {
        let mut renderer = self.renderer.lock().unwrap();
        let mut render_queue = renderer.render_queue.lock().unwrap();

        // println!("removed: {}", reference);
        render_queue.remove(reference);
    }

    pub fn input_data(&self) -> component::InputData {
        component::InputData {
            keys_pressed: self.keys_pressed.clone(),
            mouse_buttons_pressed: self.mouse_buttons_pressed.clone(),
            mouse_position: self.mouse_position,
        }
    }

    pub fn quit(&mut self) {
        exit(0);
    }

    pub fn export(&mut self) -> EngineSaveData {
        EngineSaveData::from_engine(self)
    }

    pub fn tick(&mut self) {
        if (self.paused) { return; }

        self.renderer.lock().unwrap().backend.ui_handler.clear();

        for comp in self.state.static_components.clone().iter() {
            comp.lock().unwrap().tick(self);
        }

        let mut i = 0;
        let mut obj_clone = self.state.objects.clone();
        obj_clone.reverse();

        for obj in obj_clone.iter() {
            if !gameobject::GAME_OBJECT_DESTROYED.lock().unwrap().contains(obj) {
                gameobject::to_object(*obj, |game_object| {
                    // println!("obj: {}: references: {}. engine references: {}", i, game_object.render_references.len(), self.renderer.lock().unwrap().render_queue.lock().unwrap().len());
                    game_object.render_references.reverse();
                    for ref_id in game_object.render_references.drain(..) {
                        self.remove_from_render_queue(ref_id);
                    }
                });
                i += 1;
            }
        }

        for obj in self.state.objects.clone().iter() {
            if !gameobject::GAME_OBJECT_DESTROYED.lock().unwrap().contains(obj) {
                gameobject::to_object(*obj, |game_object| {
                    if game_object.state.parent_id.is_none() {
                        game_object.tick_all(self);
                    }
                });
            }
        }
    }

    pub fn add_object(&mut self, obj: gameobject::MutexdGameObject) -> i32 {
        let id = obj.clone().lock().unwrap().id();
        self.state.add_object(id);
        self.physics_engine.add_object(id);
        id
    }

    pub fn add_static(&mut self, comp: Arc<Mutex<dyn static_component::StaticComponent>>) {
        self.state.add_static(comp);
    }

    pub fn add_ui_element(&mut self, element: UIElement) {
        self.renderer.lock().unwrap().backend.ui_handler.queue(element);
    }

    pub fn get_static_closure<T>(&mut self, mut f: impl FnMut(&mut T)) -> Option<()>
    where
        T: StaticComponent + 'static,
    {
                // Iterate over components
        let comps = self.state.static_components.clone();
        for comp_arc in comps {
            let mut _comp_lock = comp_arc.try_lock()/*.ok()?*/;

            let mut comp_lock = match _comp_lock {
                Ok(lock) => lock,
                Err(_) => {
                    println!("couldn't get lock");
                    continue;
                }
            };

            // println!("could get lock");

            // Attempt to downcast the component under the MutexGuard's scope
            // println!("{}", comp_lock.component);
            if let Some(component) = (&mut *comp_lock)
                .as_any_mut()
                .downcast_mut::<T>()
            {
                f(component); // Execute the closure with the mutable reference
                              // drop(comp_lock);
                return Some(()); // Return early on success
            }

            drop(comp_lock);
        }

        // If no component of type T was found and processed
        None
    }

    pub fn run(engine: Arc<Mutex<Self>>, event_loop: EventLoop<()>) {
        // println!("running");
        let self_clone = engine.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(16)); // Adjust timing as needed, e.g., ~60 Hz
                let mut engine = self_clone.lock().unwrap();
                engine.tick(); // Perform periodic update
                drop(engine);
            }
        });

        let self_clone_ = engine.clone();
        thread::spawn(move || {
            let mut last = SystemTime::now();
            loop {
                // let mut engine = self_clone_.try_lock().unwrap();
                // let mut dt = last.elapsed();
                // if dt.is_ok() {
                //     engine.physics_engine.tick(dt.unwrap().as_secs_f32());
                // }
                // last = SystemTime::now();
                //
                // drop(engine);
            }
        });

        let event_tx = engine.lock().unwrap().event_tx.take().unwrap();
        let control_rx = engine.lock().unwrap().control_rx.take().unwrap();
        let frame_data_rx = engine.lock().unwrap().frame_data_rx.take().unwrap();

        let win_id = engine.lock().unwrap().win_id;

        event_loop.run(move |event, _, control_flow| {
            if let Some(app_event) = AppEvent::from_event(&event, &win_id) {
                if event_tx.send(app_event.clone()).is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                let mut engine_lock = engine.lock().unwrap();

                match app_event {
                    AppEvent::KeyPressed(key) => {
                        if !engine_lock.keys_pressed.contains(&key) {
                            engine_lock.keys_pressed.push(key);
                        }
                    }
                    AppEvent::KeyReleased(key) => {
                        engine_lock.keys_pressed.retain(|&k| k != key);
                    }
                    AppEvent::MouseInput(button) => {
                        if !engine_lock.mouse_buttons_pressed.contains(&button) {
                            engine_lock.mouse_buttons_pressed.push(button);
                        }
                    }
                    AppEvent::MouseRelease(button) => {
                        engine_lock.mouse_buttons_pressed.retain(|&b| b != button);
                    }
                    AppEvent::MouseMoved(position) => {
                        engine_lock.mouse_position = position;
                    }
                    _ => {}
                }

                drop(engine_lock);
            }

            match control_rx.try_recv() {
                Ok(new_control_flow) => *control_flow = new_control_flow,
                Err(_) => *control_flow = ControlFlow::Wait,
            }

            match frame_data_rx.try_recv() {
                Ok(frame_data) => engine.lock().unwrap().dt = frame_data.dt,
                Err(_) => {}
            }
        });
    }
}
