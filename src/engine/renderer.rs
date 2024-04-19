use crate::engine::camera;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(&mut self, _cam: &camera::Camera) {}
}
