use crate::engine::component;
use std::sync::{Arc, Mutex};

pub type input = Input;

pub struct Input {
    name: String,
    state: component::ComponentState,
}

impl Input {
    pub fn new() -> Self {
        Self {
            name: String::from("Input"),
            state: component::ComponentState::new(),
        }
    }
}

impl component::ComponentTrait for Input {
    fn tick(&mut self) {
        println!("Input Tick");
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn state(&mut self) -> &mut component::ComponentState {
        &mut self.state
    }
}

pub fn make_safe<T: component::ComponentTrait + 'static>(
    comp: T,
) -> Arc<Mutex<dyn component::ComponentTrait>> {
    Arc::new(Mutex::new(comp))
}
