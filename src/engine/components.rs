use crate::engine::{component, camera};
use crate::engine::component::{
    ComponentState, ComponentTrait, ComponentWrapper, InputData, InputTickBehavior, RenderOutput,
    RenderTickBehavior, TickBehavior, TickVariant,
};
use crate::engine::gameobject::GameObject;
use crate::engine::graphics_backend::color::Colors;
use crate::engine::graphics_backend::primitives::Cube;
use crate::impl_save_load;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::graphics_backend::object::{Object, self};
use super::graphics_backend::primitives::{Primitives, self, Octagon};
use super::save::ComponentSaveLoad;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

pub struct RenderComponent {
    pub name: String,
    pub state: ComponentState,
    pub obj: Primitives
}


impl ComponentTrait for RenderComponent {
    fn name(&self) -> &str {
        &self.name
    }

    fn state(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

impl RenderTickBehavior for RenderComponent {
    fn render_tick(&mut self, obj: &mut GameObject, dt: Duration, cam: camera::Camera) -> RenderOutput {
        let mut to_render_obj: Option<Box<dyn Object>> = match &self.obj {
            Primitives::Cube(len, color) => Some(Box::new(primitives::Cube::new(*len, *color))),
            Primitives::Triangle(size, color) => Some(Box::new(primitives::Triangle::new(*size, *color))),
            Primitives::Hexagon(size, color) => Some(Box::new(primitives::Hexagon::new(*size, *color))),
            Primitives::Pentagon(size, color) => Some(Box::new(primitives::Pentagon::new(*size, *color))),
            Primitives::Octagon(size, color) => Some(Box::new(Octagon::new(*size, *color))),
            Primitives::Line(start_point, angle, length, width, color) => Some(Box::new(primitives::Line::new(start_point.clone(), *angle, *length, *width, *color))),
            Primitives::RaycastLine(start_point, angle, length, width, color) => Some(Box::new(primitives::RaycastLine::new(start_point.clone(), *angle, *length, *width, *color))),
        };

        // println!("kok: {:?}", to_render_obj.desc_raw());

        let mut out = RenderOutput {
            obj: to_render_obj
        };

        let mut pos: Option<[f32; 3]> = None;
        let mut rot: Option<[f32; 3]> = None;

        obj.get_component_closure::<component::Transform>(|comp| {
            out.obj.as_mut().expect("").move_vertexes(comp.pos.clone());
            pos = Some(comp.pos.clone());

            out.obj.as_mut().expect("").rotate_vertexes_arr(comp.rot.clone(), cam.position.into());
            rot = Some(comp.rot.clone());
        });

        // println!("{} performed a render tick on {} with transform position: {:?} and rotation: {:?}", self.name, obj.name(), pos, rot);

        out
    }
}

impl RenderComponent {
    pub fn new(obj: Primitives) -> Arc<Mutex<ComponentWrapper>> {
        let component = Arc::new(Mutex::new(Self {
            name: "RenderComponent".to_string(),
            state: ComponentState::new(),
            obj,
        }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Render(component.clone())));

        Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
    }
}

pub struct InputComponent {
    pub name: String,
    pub state: ComponentState,
}

impl ComponentTrait for InputComponent {
    fn name(&self) -> &str {
        &self.name
    }

    fn state(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

impl InputTickBehavior for InputComponent {
    fn tick_with_input(&mut self, input: &InputData, obj: &mut GameObject, dt: Duration) {}
}

impl InputComponent {
    pub fn new(name: String) -> Arc<Mutex<ComponentWrapper>> {
        let component = Arc::new(Mutex::new(Self {
            name,
            state: ComponentState::new(),
        }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Input(component.clone())));

        Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
    }
}
