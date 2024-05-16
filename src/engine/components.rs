use crate::engine::component::{
    ComponentState, ComponentTrait, ComponentWrapper, InputData, InputTickBehavior, RenderOutput,
    RenderTickBehavior, TickBehavior, TickVariant,
};
use crate::engine::graphics_backend::primitives::Cube;
use std::sync::{Arc, Mutex};
use crate::engine::gameobject::GameObject;
use crate::engine::component;

pub struct RenderComponent {
    name: String,
    state: ComponentState,
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
    fn render_tick(&mut self, obj: &mut GameObject) -> RenderOutput {
        // format!("{} performed a render tick", self.name)
        let mut out = RenderOutput {
            obj: Box::new(Cube::new(0.1, [1.0, 0.0, 0.0])),
        };
        obj.get_component_closure::<component::Transform>(|comp| {
            println!("{:?}", comp.inner.clone());
            out.obj.move_vertexes(comp.inner.clone());
        });

        out
    }
}

impl RenderComponent {
    pub fn new(name: String) -> Arc<Mutex<ComponentWrapper>> {
        let component = Arc::new(Mutex::new(Self {
            name,
            state: ComponentState::new(),
        }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Render(component.clone())));
        Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
    }
}

pub struct RenderComponent1 {
    name: String,
    state: ComponentState,
}

impl ComponentTrait for RenderComponent1 {
    fn name(&self) -> &str {
        &self.name
    }

    fn state(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

impl RenderTickBehavior for RenderComponent1 {
    fn render_tick(&mut self, obj: &mut GameObject) -> RenderOutput {
        // format!("{} performed a render tick", self.name)
        let mut out = RenderOutput {
            obj: Box::new(Cube::new(0.1, [1.0, 0.0, 0.0])),
        };
        obj.get_component_closure::<component::Transform>(|comp| {
            println!("{:?}", comp.inner.clone());
            out.obj.move_vertexes(comp.inner.clone());
        });
        out
    }
}

impl RenderComponent1 {
    pub fn new(name: String) -> Arc<Mutex<ComponentWrapper>> {
        let component = Arc::new(Mutex::new(Self {
            name,
            state: ComponentState::new(),
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
    fn tick_with_input(&mut self, input: &InputData, obj: &mut GameObject) {}
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
