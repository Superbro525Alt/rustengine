use crate::engine::camera;
use crate::engine::gameobject;
use crate::engine::renderer;
use crate::engine::static_component;
use crate::engine::component;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, Thread};

// pub trait ThreadSafe: Send + Sync {}

pub struct EngineState {
    objects: Vec<i32>,
    static_components: Vec<Arc<Mutex<dyn static_component::StaticComponent>>>,
}

// impl ThreadSafe for EngineState {}

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
    pub renderer: renderer::Renderer,
    pub graphics: bool,
    render_handle: Option<JoinHandle<()>>,
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}

// impl ThreadSafe for Engine {}

impl Engine {
    pub async fn new(graphics: bool) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            state: EngineState::new(),
            renderer: if !graphics {
                renderer::Renderer::none()
            } else {
                renderer::Renderer::new(String::from("Engine"), 800, 600).await
            },
            graphics,
            render_handle: None,
        }))
    }

    pub fn state(&mut self) -> &EngineState {
        &self.state
    }

    pub fn renderer(&mut self) -> &renderer::Renderer {
        &self.renderer
    }

    pub fn render(&mut self, data: component::RenderOutput) {

    }

    pub fn tick(&mut self) {
        // if self.render_handle.is_none() {
        //     let renderer = &self.renderer;
        //     self.render_handle = Some(thread::spawn(|| {renderer.run() }));
        // }

        for obj in self.state.objects.iter() {
            gameobject::to_object(*obj, |game_object| {
                if game_object.state.parent_id.is_none() {
                    game_object.tick_all();
                }
            });
        }

        for comp in self.state.static_components.iter_mut() {
            comp.lock().unwrap().tick();
        }

        // self.renderer.tick(&self.camera, &self.state.objects);
    }

    pub fn add_object(&mut self, obj: gameobject::MutexdGameObject) -> i32 {
        let id = obj.clone().lock().unwrap().id();
        self.state.add_object(id);
        id
    }

    pub fn add_static(&mut self, comp: Arc<dyn static_component::StaticComponent>) {
        // self.state.add_static(Arc::new(Mutex::new(comp)));
    }
}
