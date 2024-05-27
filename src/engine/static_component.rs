use downcast_rs::impl_downcast;
use downcast_rs::Downcast;
use serde::{Serializer, Serialize, Deserialize};

use super::state::Engine;

pub trait StaticComponent: Send + Sync + Downcast {
    fn tick(&mut self, engine: &mut Engine);
}

impl_downcast!(StaticComponent);

struct StaticVar {

}

impl StaticComponent for StaticVar {
    fn tick(&mut self, engine: &mut Engine) {}
}
