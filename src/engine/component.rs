use downcast_rs::impl_downcast;
use downcast_rs::Downcast;
use serde_json::Value;
use std::any::{self, Any, TypeId};
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

pub trait ComponentTrait: Send + Sync + Downcast
where
    Self: 'static,
{
    fn tick(&mut self);
    fn name(&self) -> &str;
    fn state(&mut self) -> &mut ComponentState;
}

impl_downcast!(ComponentTrait);

pub struct LambdaComponent<F, T>
where
    F: FnMut() -> T,
{
    name: String,
    state: ComponentState,
    tick: F,
}

impl<F, T> ComponentTrait for LambdaComponent<F, T>
where
    Self: 'static,
    F: FnMut() -> T + Send + Sync,
{
    fn tick(&mut self) {
        (self.tick)(); // Call the closure
        println!("Tick on {}.", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn state(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

impl<F, T> LambdaComponent<F, T>
where
    F: FnMut() -> T + Send + Sync,
{
    pub fn new(name: String, f: F) -> Self {
        Self {
            name,
            tick: f,
            state: ComponentState::new(),
        }
    }
}
