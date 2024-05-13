use crate::engine::component::{ComponentTrait, TickVariant, TickBehavior, InputTickBehavior, InputData, RenderTickBehavior, RenderOutput, ComponentState, ComponentWrapper};
use std::sync::{Arc, Mutex};
use crate::engine::graphics_backend::primitives::Cube;

pub struct RenderComponent {
    name: String,
    state: ComponentState
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
    fn render_tick(&mut self) -> RenderOutput {
        // format!("{} performed a render tick", self.name)
        RenderOutput{obj: Box::new(Cube::new(0.1, [1.0, 0.0, 0.0]))}
    }
}

impl RenderComponent {
    pub fn new(name: String) -> Arc<Mutex<ComponentWrapper>> {
        let component = Arc::new(Mutex::new(Self { name, state: ComponentState::new() }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Render(component.clone())));
        Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
    }
}

pub struct InputComponent {
    name: String,
    state: ComponentState
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
    fn tick_with_input(&mut self, input: &InputData) {
    }
}

impl InputComponent {
    pub fn new(name: String) -> Arc<Mutex<ComponentWrapper>> {
        let component = Arc::new(Mutex::new(Self { name, state: ComponentState::new() }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Input(component.clone())));
        Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
    }
}


