use crate::engine::camera;
use crate::engine::gameobject;
use crate::engine::renderer;
use crate::engine::static_component;
use crate::engine::component;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::thread::{JoinHandle};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

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

    pub fn objects(&mut self) -> &Vec<i32> {
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
    pub event_loop: EventLoop<()>,
    pub graphics: bool,
    render_handle: Option<JoinHandle<()>>,
    event_tx: Option<Sender<Event<()>>>,
    control_rx: Option<mpsc::Receiver<ControlFlow>>,
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}

impl Engine {
    pub async fn new(graphics: bool) -> Self {
        let (renderer, event_loop) = renderer::Renderer::new(String::from("Engine"), 800, 600).await;
        let renderer = Arc::new(Mutex::new(renderer));

        let (event_tx, event_rx) = mpsc::channel();
        let (control_tx, control_rx) = mpsc::channel();

        let mut engine = Self {
            state: EngineState::new(),
            renderer: renderer.clone(),
            event_loop,
            graphics,
            render_handle: None,
            event_tx: Some(event_tx),
            control_rx: Some(control_rx),
        };

        if graphics {
            let renderer_clone = renderer.clone();
            engine.render_handle = Some(thread::spawn(move || {
                renderer::Renderer::run(renderer_clone, event_rx, control_tx);
            }));
        }

        engine
    }

    pub fn state(&mut self) -> &EngineState {
        &self.state
    }

    pub fn renderer(&mut self) -> Arc<Mutex<renderer::Renderer>> {
        self.renderer.clone()
    }

    pub fn render(&mut self, data: component::RenderOutput) -> usize {
        self.renderer.lock().unwrap().render_queue.lock().unwrap().push(data);
        let pos = self.renderer.lock().unwrap().render_queue.lock().unwrap().len() - 1;
        pos 
    }

    pub fn remove_from_render_queue(&mut self, reference: usize) {
        self.renderer.lock().unwrap().render_queue.lock().unwrap().remove(reference); 
    }
    
    pub fn input_data(&mut self) -> component::InputData {
        component::InputData{}
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

    pub fn add_static(&mut self, comp: Arc<dyn static_component::StaticComponent>) {
        // self.state.add_static(Arc::new(Mutex::new(comp)));
    }

    pub fn run(&mut self) {
        if self.graphics {
            let event_tx = self.event_tx.take().unwrap();
            let control_rx = self.control_rx.take().unwrap();
            let event_loop = &self.event_loop;

            event_loop.run(move |event, _, control_flow| {
                if event_tx.send(event).is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Check control flow from renderer
                match control_rx.try_recv() {
                    Ok(new_control_flow) => *control_flow = new_control_flow,
                    Err(_) => *control_flow = ControlFlow::Wait,
                }
            });
        }
    }
}

