use crate::engine::component;
use crate::engine::component::{
    ComponentState, ComponentTrait, ComponentWrapper, InputData, InputTickBehavior, RenderOutput,
    RenderTickBehavior, TickBehavior, TickVariant,
};
use crate::engine::gameobject::GameObject;
use crate::engine::graphics_backend::color::Colors;
use crate::engine::graphics_backend::primitives::Cube;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct RenderComponent {
    name: String,
    state: ComponentState,
    color: Colors,
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
    fn render_tick(&mut self, obj: &mut GameObject, dt: Duration) -> RenderOutput {
        let mut out = RenderOutput {
            obj: Box::new(Cube::new(0.1, self.color.raw())),
        };

        let mut pos: Option<[f32; 3]> = None;

        obj.get_component_closure::<component::Transform>(|comp| {
            out.obj.move_vertexes(comp.inner.clone());
            pos = Some(comp.inner.clone());
        });

        // println!("{} performed a render tick on {} with transform {:?}", self.name, obj.name(), pos);

        out
    }
}

impl RenderComponent {
    pub fn new(name: String, color: Colors) -> Arc<Mutex<ComponentWrapper>> {
        let component = Arc::new(Mutex::new(Self {
            name,
            state: ComponentState::new(),
            color,
        }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Render(component.clone())));
        Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
    }
}

pub struct InputComponent {
    name: String,
    state: ComponentState,
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
