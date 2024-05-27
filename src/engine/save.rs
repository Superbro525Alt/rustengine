use downcast_rs::Downcast;
use winit::event_loop::{EventLoopBuilder, EventLoop};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::engine::component::{ComponentWrapper, ComponentTrait};
use crate::engine::collider::{Collider, CubeCollider, RectangularPrismCollider, PointCollider, OctagonCollider, Point};
use crate::engine::bounds::{Bounds2D, Bounds3D, Limits2D, Limits3D};
use crate::engine::gameobject::{GameObject, GameObjectState};
use std::collections::HashMap;
use lazy_static::lazy_static;
use super::state::Engine;
use super::static_component::StaticComponent;

#[derive(Serialize, Deserialize, Debug)]
pub struct EngineSaveData {
    objects: Vec<GameObjectSaveData>,
    static_components: Vec<StaticComponentSaveData>,
    graphics: bool
}

impl EngineSaveData {
    pub fn from_engine(e: &mut Engine) -> EngineSaveData {
        Self {
            objects: e.state.objects().iter_mut().map(|obj| {
                GameObjectSaveData::from_game_object(&mut GameObject::find_by_id(*obj).expect("cannot find associated object").lock().unwrap())
            }).collect(),
            static_components: Vec::new(),
            graphics: e.graphics
        }

    }

    pub async fn to_engine(&mut self) -> (Engine, EventLoop<()>) {
        let e = self;

        let event_loop = EventLoopBuilder::<()>::with_user_event().build();
        
        let mut engine = Engine::new(e.graphics, event_loop).await;
    
        for obj in e.objects.iter_mut() {
            engine.0.add_object(obj.to_game_object());
        }

        for static_comp in e.static_components.iter_mut() {
            // engine.0.add_static(static_comp.to_component());
        }

        engine
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GameObjectSaveData {
    components: Vec<ComponentSaveData>,
    colliders: Vec<ColliderSaveData>,
    parent: Option<i32>,
    children: Vec<i32>,
    id: i32,
    name: String,
    active: bool,
}

impl GameObjectSaveData {
    pub fn from_game_object(obj: &GameObject) -> Self {
        GameObjectSaveData {
            // components: obj.components.iter().map(|c| c.lock().unwrap().serialize()).collect(),
            // colliders: obj.colliders.iter().map(|c| ColliderSaveData::from_collider(c.clone())).collect(),
            components: Vec::new(),
            colliders: Vec::new(),
            parent: obj.state.parent_id,
            children: obj.state.child_ids.clone(),
            id: obj.id,
            name: obj.name.clone(),
            active: obj.state.active,
        }
    }

    pub fn to_game_object(&self) -> Arc<Mutex<GameObject>> {
        let obj = GameObject::new(self.name.clone(), vec![], GameObjectState::new(self.active, self.parent, self.children.clone()));

        for comp in &self.components {
            // obj.lock().unwrap().add_component(ComponentWrapper::deserialize(comp.clone()));
        }

        for coll in &self.colliders {
            obj.lock().unwrap().add_collider(Arc::new(Mutex::new(coll.to_collider())));
        }

        obj
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ComponentSaveData {
    
}

#[derive(Serialize, Deserialize, Debug)]
struct StaticComponentSaveData {
}

// impl StaticComponentSaveData {
//     pub fn to_static(&mut self) -> Arc<Mutex<dyn StaticComponent>> {
//     }
//
//     pub fn from_static(s: Arc<Mutex<dyn StaticComponent>>) -> Self {
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub enum ColliderType {
    CubeCollider { side_length: f32 },
    RectangularPrismCollider { width: f32, height: f32, depth: f32 },
    PointCollider { point: Point },
    OctagonCollider { size: f32 },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColliderSaveData {
    collider: ColliderType,
}

impl ColliderSaveData {
    pub fn from_collider(collider: Arc<Mutex<dyn Collider>>) -> Self {
        let lock = collider.lock().unwrap();
        let collider_type = if let Some(cube) = lock.downcast_ref::<CubeCollider>() {
            ColliderType::CubeCollider { side_length: cube.side_length }
        } else if let Some(rect) = lock.downcast_ref::<RectangularPrismCollider>() {
            ColliderType::RectangularPrismCollider { width: rect.width, height: rect.height, depth: rect.depth }
        } else if let Some(point) = lock.downcast_ref::<PointCollider>() {
            ColliderType::PointCollider { point: point.point.clone() }
        } else if let Some(octagon) = lock.downcast_ref::<OctagonCollider>() {
            ColliderType::OctagonCollider { size: octagon.size }
        } else {
            panic!("Unknown collider type");
        };
        
        ColliderSaveData { collider: collider_type }
    }

    pub fn to_collider(&self) -> Box<dyn Collider> {
        match &self.collider {
            ColliderType::CubeCollider { side_length } => CubeCollider::new(*side_length),
            ColliderType::RectangularPrismCollider { width, height, depth } => RectangularPrismCollider::new(*width, *height, *depth),
            ColliderType::PointCollider { point } => PointCollider::new(point.clone()),
            ColliderType::OctagonCollider { size } => OctagonCollider::new(*size),
        }
    }
}

