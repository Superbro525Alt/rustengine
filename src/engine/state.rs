use crate::engine::camera;
use crate::engine::gameobject;
use crate::engine::renderer;
use crate::engine::static_component;
use std::sync::{Arc, Mutex};

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
    pub camera: camera::Camera,
    pub renderer: renderer::Renderer,
}

impl Engine {
    pub async fn new() -> Self {
        Self {
            state: EngineState::new(),
            camera: camera::Camera::new(),
            // renderer: renderer::Renderer::none()
            renderer: renderer::Renderer::new(String::from("Engine"), 800, 600).await,
        }
    }

    pub fn state(&mut self) -> &EngineState {
        &self.state
    }

    pub fn camera(&mut self) -> &camera::Camera {
        &self.camera
    }

    pub fn renderer(&mut self) -> &renderer::Renderer {
        &self.renderer
    }

    pub fn tick(&mut self) {
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
        self.camera.tick();
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
