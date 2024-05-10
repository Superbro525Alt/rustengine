use crate::engine::component::{ComponentTrait, TickVariant, TickBehavior, InputTickBehavior, InputData, RenderTickBehavior, RenderOutput, ComponentState, ComponentWrapper};
use std::sync::{Arc, Mutex};

struct RenderComponent {
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
        RenderOutput{}
    }
}

impl RenderComponent {
    pub fn new(name: String) -> Arc<Mutex<ComponentWrapper>> {
        let component = Arc::new(Mutex::new(RenderComponent { name, state: ComponentState::new() }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Render(component.clone())));
        Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
    }
}

