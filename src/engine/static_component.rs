use downcast_rs::impl_downcast;
use downcast_rs::Downcast;
use serde::{Serializer, Serialize, Deserialize};
use std::sync::{Arc, Mutex};

use super::save::StaticComponentSaveLoad;
use super::state::Engine;

pub trait StaticComponent: Send + Sync + Downcast + StaticComponentSaveLoad 
where Self: 'static {
    fn tick(&mut self, engine: &mut Engine);
    fn name(&mut self) -> String;
}

pub struct Container {
    pub internal: Arc<Mutex<dyn StaticComponent>>
}

impl_downcast!(StaticComponent);
