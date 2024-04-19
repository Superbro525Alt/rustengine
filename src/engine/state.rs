use crate::engine::gameobject;
use crate::engine::camera;
use crate::engine::renderer;

pub struct State {
    objects: Vec<i32>,
}

impl State {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    
    pub fn objects(&mut self) -> &Vec<i32> {
        &self.objects
    }

    pub fn add_object(&mut self, obj: i32) {
        self.objects.push(obj);
    }
}

pub struct Engine {
    state: State,
    camera: camera::Camera,
    renderer: renderer::Renderer
}

impl Engine {
    pub fn new() -> Self {
        Self { state: State::new(), camera: camera::Camera::new(), renderer: renderer::Renderer::new() }
    }
    pub fn state(&mut self) -> &State {
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

        self.renderer.tick(&self.camera);
        self.camera.tick();
    }

    pub fn add_object(&mut self, obj: gameobject::game_object) -> i32 {
        let id = obj.clone().lock().unwrap().id();
        self.state.add_object(id);
        id
    }
}
