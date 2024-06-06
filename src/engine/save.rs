use downcast_rs::{impl_downcast, Downcast};
pub use uuid::Uuid;
use crate::engine::static_component::Container;
use std::sync::{Arc, Mutex};
// use uuid::Uuid;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
// use downcast_rs::{impl_downcast, Downcast};
// use crate::engine::static_component::Container;
use crate::engine::component::ComponentState;
use winit::event_loop::{EventLoopBuilder, EventLoop};
use std::sync::{RwLock};
use serde_json::Value;
use crate::engine::component::{ComponentWrapper, ComponentTrait, TickBehavior, Transform};
use crate::engine::collider::{Collider, CubeCollider, RectangularPrismCollider, PointCollider, OctagonCollider, Point};
use crate::engine::bounds::{Bounds2D, Bounds3D, Limits2D, Limits3D};
use crate::engine::gameobject::{GameObject, GameObjectState};
use std::collections::HashMap;
use lazy_static::lazy_static;
use super::component::{TickVariant, InputTickBehavior, RenderTickBehavior, self, CharacterController2D};
use super::components::{InputComponent, RenderComponent};
use super::graphics_backend::primitives::Primitives;
use super::state::Engine;
pub use super::static_component::StaticComponent;
use std::any::Any;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt::Debug;
// use std::sync::{Arc, Mutex};
use std::hash::{Hash, Hasher};


// Wrapper struct for Arc<Mutex<dyn StaticComponent>>
#[derive(Clone)]
pub struct StaticComponentKey {
    pub inner: Arc<Mutex<dyn StaticComponent>>,
}

impl PartialEq for StaticComponentKey {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for StaticComponentKey {}

impl Hash for StaticComponentKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr: *const _ = Arc::as_ptr(&self.inner);
        ptr.hash(state);
    }
}

impl StaticComponentKey {
    pub fn new(component: Arc<Mutex<dyn StaticComponent>>) -> Self {
        Self { inner: component }
    }
}

#[derive(Debug, Clone)]
pub struct Link<T> {
    pub id: Option<Uuid>,
    pub data: Arc<Mutex<T>>,
}

impl<T: Default + Clone + Send + Sync + 'static> Default for Link<T> {
    fn default() -> Self {
        Self {
            id: Some(Uuid::new_v4()),
            data: Arc::new(Mutex::new(T::default())),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Link<T> {
    pub fn new(data: impl Into<Arc<Mutex<T>>>) -> Self {
        let data = data.into();
        let id = Uuid::new_v4();
        let link = Self { id: Some(id), data };
        register_link(link.clone());
        link
    }

    pub fn clone_with_uuid(&self, uuid: Uuid) -> Self {
        Self {
            id: Some(uuid),
            data: Arc::clone(&self.data),
        }
    }
}

// Serialization and deserialization for Link
impl<T: Serialize + Clone + Debug + Send + Sync + 'static> Serialize for Link<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.id.unwrap().to_string())
    }
}

impl<'de, T: Deserialize<'de> + Clone + Debug + Default + Send + Sync + 'static> Deserialize<'de> for Link<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id_str = String::deserialize(deserializer)?;
        let id = Uuid::parse_str(&id_str).map_err(serde::de::Error::custom)?;
        if let Some(link) = get_link::<T>(Some(id)) {
            Ok(link)
        } else {
            let new_link = Link::default();
            register_link(new_link.clone());
            Ok(new_link)
        }
    }
}

// Define a trait for serialization and deserialization
pub(crate) trait ComponentSaveLoad: Send + Sync + Downcast + Any {
    fn to_save_data(&self) -> Value;
    fn from_save_data(data: Value) -> Arc<Mutex<ComponentWrapper>>
    where
        Self: Sized;
}

pub(crate) trait StaticComponentSaveLoad: Send + Sync + Downcast + Any {
    fn to_save_data(&self) -> Value;
    fn from_save_data(data: Value) -> Container
    where
        Self: Sized;
}

impl_downcast!(ComponentSaveLoad);
impl_downcast!(StaticComponentSaveLoad);

lazy_static! {
    static ref COMPONENT_REGISTRY: RwLock<HashMap<String, Box<dyn Fn(Value) -> Arc<Mutex<ComponentWrapper>> + Send + Sync>>> = RwLock::new(HashMap::new());
    static ref STATIC_COMPONENT_REGISTRY: RwLock<HashMap<String, Box<dyn Fn(Value) -> Arc<Mutex<dyn StaticComponent>> + Send + Sync>>> = RwLock::new(HashMap::new());
    static ref LINK_REGISTRY: RwLock<HashMap<Uuid, Box<dyn Any + Send + Sync>>> = RwLock::new(HashMap::new());
    pub static ref UUID_REGISTRY: RwLock<HashMap<StaticComponentKey, Uuid>> = RwLock::new(HashMap::new());
}

pub fn register_link<T: 'static + Send + Sync>(link: Link<T>) {
    let mut registry = LINK_REGISTRY.write().unwrap();
    registry.insert(link.id.unwrap(), Box::new(link.data) as Box<dyn Any + Send + Sync>);
}

pub fn get_link<T: 'static + Send + Sync>(id: Option<Uuid>) -> Option<Link<T>> {
    println!("ok {:?}", id);
    if let Some(id) = id {
    let registry = LINK_REGISTRY.read().unwrap();
        return registry.get(&id).and_then(|data| data.downcast_ref::<Arc<Mutex<T>>>().cloned().map(|data| Link { id: Some(id), data }));
    } else {
        return None;
    }
}

#[macro_export]
macro_rules! impl_save_load_default {
    ($comp_type:ty, $save_struct:ident, { $( $field:ident : $field_type:ty ),* }, { $( $link_field:ident : $link_field_type:ty ),* }) => {
        #[derive(Serialize, Deserialize, Debug, Default)]
        pub(crate) struct $save_struct {
            uuid: Option<Uuid>,
            $( $field: $field_type ),*,
            $( $link_field: $crate::engine::save::Link<$link_field_type> ),*
        }

        impl $crate::engine::save::ComponentSaveLoad for $comp_type {
            fn to_save_data(&self) -> serde_json::Value {
                let mut uuid_registry = $crate::engine::save::UUID_REGISTRY.write().unwrap();
                let save_data = $save_struct {
                    uuid: None,
                    $( $field: self.$field.clone() ),*,
                    $( $link_field: {
                        let link = &self.$link_field;
                        let arc_mutex = StaticComponentKey::new(link.data.clone() as Arc<Mutex<dyn StaticComponent>>);
                        if let Some(existing_uuid) = uuid_registry.get(&arc_mutex) {
                            link.clone_with_uuid(*existing_uuid)
                        } else {
                            let new_uuid = Uuid::new_v4();
                            uuid_registry.insert(arc_mutex, new_uuid);
                            link.clone_with_uuid(new_uuid)
                        }
                    }),*
                };
                serde_json::to_value(save_data).unwrap()
            }

            fn from_save_data(data: serde_json::Value) -> Arc<Mutex<ComponentWrapper>> {
                let save_data: $save_struct = serde_json::from_value(data).unwrap();
                let component = Arc::new(Mutex::new(Self {
                    $( $field: save_data.$field ),*,
                    $( $link_field: $crate::engine::save::get_link(Some(save_data.$link_field.id.unwrap())).unwrap_or_else(|| {
                        let new_link = $crate::engine::save::Link::new(Arc::new(Mutex::new(<$link_field_type>::default())));
                        $crate::engine::save::register_link(new_link.clone());
                        new_link
                    })),*
                }));

                let ticker = Arc::new(Mutex::new(TickVariant::Default(component.clone() as Arc<Mutex<dyn $crate::engine::component::TickBehavior>>)));

                Arc::new(Mutex::new($crate::engine::component::ComponentWrapper::new(component as Arc<Mutex<dyn $crate::engine::component::ComponentTrait>>, ticker)))
            }
        }
        $crate::engine::save::register_component::<$comp_type>(stringify!($comp_type));
    };
}

#[macro_export]
macro_rules! impl_save_load_input {
    ($comp_type:ty, $save_struct:ident, { $( $field:ident : $field_type:ty ),* }, { $( $link_field:ident : $link_field_type:ty ),* }) => {
        #[derive(Serialize, Deserialize, Debug, Default)]
        pub(crate) struct $save_struct {
            uuid: Option<Uuid>,
            $( $field: $field_type ),*,
            $( $link_field: $crate::engine::save::Link<$link_field_type> ),*
        }

        impl $crate::engine::save::ComponentSaveLoad for $comp_type {
            fn to_save_data(&self) -> serde_json::Value {
                let mut uuid_registry = $crate::engine::save::UUID_REGISTRY.write().unwrap();
                let save_data = $save_struct {
                    uuid: None,
                    $( $field: self.$field.clone() ),*,
                    $( $link_field: {
                        let link = &self.$link_field;
                        let arc_mutex = StaticComponentKey::new(link.data.clone() as Arc<Mutex<dyn StaticComponent>>);
                        if let Some(existing_uuid) = uuid_registry.get(&arc_mutex) {
                            link.clone_with_uuid(*existing_uuid)
                        } else {
                            let new_uuid = Uuid::new_v4();
                            uuid_registry.insert(arc_mutex, new_uuid);
                            link.clone_with_uuid(new_uuid)
                        }
                    }),*
                };
                serde_json::to_value(save_data).unwrap()
            }

            fn from_save_data(data: serde_json::Value) -> Arc<Mutex<ComponentWrapper>> {
                let save_data: $save_struct = serde_json::from_value(data).unwrap();
                let component = Arc::new(Mutex::new(Self {
                    $( $field: save_data.$field ),*,
                    $( $link_field: $crate::engine::save::get_link(Some(save_data.$link_field.id.unwrap())).unwrap_or_else(|| {
                        let new_link = $crate::engine::save::Link::new(Arc::new(Mutex::new(<$link_field_type>::default())));
                        $crate::engine::save::register_link(new_link.clone());
                        new_link
                    })),*
                }));

                let ticker = Arc::new(Mutex::new(TickVariant::Input(component.clone() as Arc<Mutex<dyn $crate::engine::component::InputTickBehavior>>)));

                Arc::new(Mutex::new($crate::engine::component::ComponentWrapper::new(component as Arc<Mutex<dyn $crate::engine::component::ComponentTrait>>, ticker)))
            }
        }
        $crate::engine::save::register_component::<$comp_type>(stringify!($comp_type));
    };
}

#[macro_export]
macro_rules! impl_save_load_render {
    ($comp_type:ty, $save_struct:ident, { $( $field:ident : $field_type:ty ),* }, { $( $link_field:ident : $link_field_type:ty ),* }) => {
        #[derive(Serialize, Deserialize, Debug, Default)]
        pub(crate) struct $save_struct {
            uuid: Option<Uuid>,
            $( $field: $field_type ),*,
            $( $link_field: $crate::engine::save::Link<$link_field_type> ),*
        }

        impl $crate::engine::save::ComponentSaveLoad for $comp_type {
            fn to_save_data(&self) -> serde_json::Value {
                let mut uuid_registry = $crate::engine::save::UUID_REGISTRY.write().unwrap();
                let save_data = $save_struct {
                    uuid: None,
                    $( $field: self.$field.clone() ),*,
                    $( $link_field: {
                        let link = &self.$link_field;
                        let arc_mutex = StaticComponentKey::new(link.data.clone() as Arc<Mutex<dyn StaticComponent>>);
                        if let Some(existing_uuid) = uuid_registry.get(&arc_mutex) {
                            link.clone_with_uuid(*existing_uuid)
                        } else {
                            let new_uuid = Uuid::new_v4();
                            uuid_registry.insert(arc_mutex, new_uuid);
                            link.clone_with_uuid(new_uuid)
                        }
                    }),*
                };
                serde_json::to_value(save_data).unwrap()
            }

            fn from_save_data(data: serde_json::Value) -> Arc<Mutex<ComponentWrapper>> {
                let save_data: $save_struct = serde_json::from_value(data).unwrap();
                let component = Arc::new(Mutex::new(Self {
                    $( $field: save_data.$field ),*,
                    $( $link_field: $crate::engine::save::get_link(save_data.$link_field.id.unwrap()).unwrap_or_else(|| {
                        let new_link = $crate::engine::save::Link::new(Arc::new(Mutex::new(<$link_field_type>::default())));
                        $crate::engine::save::register_link(new_link.clone());
                        new_link
                    })),*
                }));

                let ticker = Arc::new(Mutex::new(TickVariant::Render(component.clone() as Arc<Mutex<dyn $crate::engine::component::RenderTickBehavior>>)));

                Arc::new(Mutex::new($crate::engine::component::ComponentWrapper::new(component as Arc<Mutex<dyn $crate::engine::component::ComponentTrait>>, ticker)))
            }
        }
        $crate::engine::save::register_component::<$comp_type>(stringify!($comp_type));
    };
}

    #[macro_export]
macro_rules! impl_static_save_load {
    ($comp_type:ty, $save_struct:ident, { $( $field:ident : $field_type:ty ),* }, { $( $link_field:ident : $link_field_type:ty ),* }) => {
        #[derive(Serialize, Deserialize, Debug, Default)]
        pub(crate) struct $save_struct {
            uuid: Option<Uuid>,
            $( $field: $field_type ),*,
            $( $link_field: $crate::engine::save::Link<$link_field_type> ),*
        }

        impl $crate::engine::save::StaticComponentSaveLoad for $comp_type {
            fn to_save_data(&self) -> serde_json::Value {
                let mut uuid_registry = $crate::engine::save::UUID_REGISTRY.write().unwrap();
                let arc_mutex = StaticComponentKey::new(Arc::new(Mutex::new(self.clone())));

                {
                    println!("{}, {:?}", stringify!($save_struct), uuid_registry.contains_key(&arc_mutex.clone()));
                }

                let uuid = uuid_registry.entry(arc_mutex.clone()).or_insert_with(Uuid::new_v4);
                let save_data = $save_struct {
                    uuid: Some(*uuid),
                    $( $field: self.$field.clone() ),*,
                    $( $link_field: {
                        let link = &self.$link_field;
                        let arc_mutex = StaticComponentKey::new(link.data.clone() as Arc<Mutex<dyn StaticComponent>>);
                        if let Some(existing_uuid) = uuid_registry.get(&arc_mutex) {
                            link.clone_with_uuid(*existing_uuid)
                        } else {
                            let new_uuid = Uuid::new_v4();
                            uuid_registry.insert(arc_mutex, new_uuid);
                            link.clone_with_uuid(new_uuid)
                        }
                    }),*
                };
                serde_json::to_value(save_data).unwrap()
            }

            fn from_save_data(data: serde_json::Value) -> $crate::engine::static_component::Container {
                let save_data: $save_struct = serde_json::from_value(data).unwrap();
                let component = Arc::new(Mutex::new(Self {
                    $( $field: save_data.$field ),*,
                    $( $link_field: $crate::engine::save::get_link(save_data.$link_field.id).unwrap_or_else(|| {
                        let new_link = $crate::engine::save::Link::new(Arc::new(Mutex::new(<$link_field_type>::default())));
                        $crate::engine::save::register_link(new_link.clone());
                        new_link
                    })),*
                }));

                $crate::engine::static_component::Container {
                    internal: component
                }
            }
        }

        $crate::engine::save::register_static_component::<$comp_type>(stringify!($comp_type));
    };
}

#[macro_export]
macro_rules! impl_save_load {
    ($comp_type:ty, $save_struct:ident, default, { $( $field:ident : $field_type:ty ),* }, { $( $link_field:ident : $link_field_type:ty ),* }) => {
        impl_save_load_default!($comp_type, $save_struct, { $( $field : $field_type ),* }, { $( $link_field : $link_field_type ),* });
    };
    ($comp_type:ty, $save_struct:ident, input, { $( $field:ident : $field_type:ty ),* }, { $( $link_field:ident : $link_field_type:ty ),* }) => {
        impl_save_load_input!($comp_type, $save_struct, { $( $field : $field_type ),* }, { $( $link_field : $link_field_type ),* });
    };
    ($comp_type:ty, $save_struct:ident, render, { $( $field:ident : $field_type:ty ),* }, { $( $link_field:ident : $link_field_type:ty ),* }) => {
        impl_save_load_render!($comp_type, $save_struct, { $( $field : $field_type ),* }, { $( $link_field : $link_field_type ),* });
    };
}

pub fn init() {
    impl_save_load!(
        Transform, 
        TransformSaveData, 
        default, 
        { pos: [f32; 3], rot: [f32; 3], state: ComponentState },
        { }
    );
    
    impl_save_load!(
        CharacterController2D, 
        CharacterController2DSaveData, 
        input, 
        { moveamt: f32, rotamt: f32, bounds: Option<Bounds2D>, state: ComponentState },
        { }
    );
    
    impl_save_load!(
        InputComponent, 
        InputComponentSaveData, 
        input, 
        { name: String, state: ComponentState },
        { }
    );
    
    impl_save_load!(
        RenderComponent, 
        RenderComponentSaveData, 
        render, 
        { name: String, obj: Primitives, state: ComponentState },
        { }
    );
}

pub fn register_component<T: ComponentSaveLoad + 'static>(name: &str) {
    let mut registry = COMPONENT_REGISTRY.write().unwrap();
    registry.insert(
        name.to_string(),
        Box::new(|data: Value| -> Arc<Mutex<ComponentWrapper>> {
            T::from_save_data(data)
        }) as Box<dyn Fn(Value) -> Arc<Mutex<ComponentWrapper>> + Send + Sync>,
    );
}

pub fn register_static_component<T: StaticComponentSaveLoad + 'static>(name: &str) {
    let mut registry = STATIC_COMPONENT_REGISTRY.write().unwrap();
    registry.insert(
        name.to_string(),
        Box::new(|data: Value| -> Arc<Mutex<dyn StaticComponent>> {
            T::from_save_data(data).internal
        }) as Box<dyn Fn(Value) -> Arc<Mutex<dyn StaticComponent>> + Send + Sync>,
    );
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EngineSaveData {
    pub objects: Vec<GameObjectSaveData>,
    pub static_components: Vec<StaticComponentSaveData>,
    pub graphics: bool,
}

impl EngineSaveData {
    pub fn from_engine(e: &mut Engine) -> EngineSaveData {
        Self {
            objects: e.state.objects().iter_mut().map(|obj| {
                GameObjectSaveData::from_game_object(&mut GameObject::find_by_id(*obj).expect("cannot find associated object").lock().unwrap())
            }).collect(),
            static_components: e.state.static_components.iter().map(|static_comp| {
                StaticComponentSaveData::from_static_component(static_comp.clone())
            }).collect(),
            graphics: e.graphics,
        }
    }

    pub fn from_engine_to_json(e: &mut Engine) -> String {
        let save = Self::from_engine(e);
        serde_json::to_string(&save).unwrap()
    }

    pub async fn to_engine(&mut self) -> (Engine, EventLoop<()>) {
        let event_loop = EventLoopBuilder::<()>::with_user_event().build();
        let mut engine = Engine::new(self.graphics, event_loop).await;
    
        for obj in self.objects.iter_mut() {
            engine.0.add_object(obj.to_game_object());
        }

        for static_comp in self.static_components.iter_mut() {
            engine.0.add_static(static_comp.to_static_component());
        }

        engine
    }

    pub async fn to_engine_from_data(data: String) -> (Engine, EventLoop<()>) {
        let save = serde_json::from_str(&data).unwrap();
        let mut e = serde_json::from_value::<Self>(save).unwrap();
        Self::to_engine(&mut e).await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameObjectSaveData {
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
            components: obj.components.iter().map(|c| ComponentSaveData::from_component(c.clone())).collect(),
            colliders: obj.colliders.iter().map(|c| ColliderSaveData::from_collider(c.clone())).collect(),
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
            obj.lock().unwrap().add_component(comp.to_component());
        }

        for coll in &self.colliders {
            obj.lock().unwrap().add_collider(Arc::new(Mutex::new(coll.to_collider())));
        }

        obj
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentSaveData {
    id: String,
    data: Value,
}

impl ComponentSaveData {
    pub fn from_component(comp: Arc<Mutex<ComponentWrapper>>) -> Self {
        let mut lock = match comp.try_lock() {
            Ok(lock) => lock,
            Err(_) => {
                panic!("couldn't get lock");
            }
        };

        let comp_lock = (&mut *lock.component.lock().unwrap());

        let (name, data) = {
            let name = comp_lock.name().to_string();
            
            let data = ComponentTrait::to_save_data(comp_lock);

            (name, data)
        };

        Self { id: name, data }
    }

    pub fn to_component(&self) -> Arc<Mutex<ComponentWrapper>> {
        let registry = COMPONENT_REGISTRY.read().unwrap();
        if let Some(constructor) = registry.get(&self.id) {
            constructor(self.data.clone())
        } else {
            panic!("Unknown component type: {}", self.id);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StaticComponentSaveData {
    id: String,
    data: Value,
}

impl StaticComponentSaveData {
    pub fn from_static_component(comp: Arc<Mutex<dyn StaticComponent>>) -> Self {
        let mut lock = comp.lock().unwrap();
        let data = lock.to_save_data();

        Self { id: lock.name(), data }
    }

    pub fn to_static_component(&self) -> Arc<Mutex<dyn StaticComponent>> {
        let registry = STATIC_COMPONENT_REGISTRY.read().unwrap();
        println!("{:?}", registry.iter().map(|a| {a.0}).collect::<Vec<_>>());
        if let Some(constructor) = registry.get(&self.id) {
            constructor(self.data.clone())
        } else {
            panic!("Unknown static component type: {}", self.id);
        }
    }
}

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
    pub fn from_collider(collider: Arc<Mutex<Box<dyn Collider>>>) -> Self {
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
