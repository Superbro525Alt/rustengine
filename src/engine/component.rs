use serde_json::Value;
use std::any::TypeId;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ComponentState {
    _state: Value,
}

impl ComponentState {
    pub fn new() -> Self {
        Self {
            _state: Value::Null,
        }
    }
    pub fn get(&self) -> Value {
        self._state.clone()
    }
    pub fn set(&mut self, value: Value) {
        self._state = value;
    }
}

pub trait ComponentTrait: Send + Sync
where
    Self: 'static,
{
    fn tick(&mut self);
    fn name(&self) -> &str;
    fn state(&mut self) -> &mut ComponentState;
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

pub struct Component {
    name: String,
    state: ComponentState,
}

impl ComponentTrait for Component {
    fn tick(&mut self) {
        // Logic to tick the component
        println!("Tick on {}.", self.name())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn state(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

pub fn create_component(name: String, _deps: Vec<Arc<Mutex<dyn ComponentTrait>>>) -> Component {
    Component {
        name,
        state: ComponentState::new(),
    }
}

pub fn to_component<F, T>(object: Arc<Mutex<Component>>, f: F) -> T
where
    F: FnOnce(&mut Component) -> T,
{
    let mut comp = object.lock().unwrap();
    f(&mut comp)
}
