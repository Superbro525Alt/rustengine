use crate::engine::component;
use crate::engine::utils;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::any::{TypeId, type_name, Any};

pub type game_object = Arc<Mutex<GameObject>>;

lazy_static! {
    pub static ref GAME_OBJECT_REGISTRY: Mutex<HashMap<i32, Arc<Mutex<GameObject>>>> = Mutex::new(HashMap::new());
    static ref GAME_OBJECT_COUNT: Mutex<Count> = Mutex::new(Count::new());
}

#[derive(Clone)]
struct Count {
    internal: i32
}

impl Count {
    pub fn new() -> Self {
        Self { internal: 0 }
    }

    pub fn increment(&mut self) {
        self.internal += 1;
    }

    pub fn get(&mut self) -> i32 {
        self.internal
    }
}

#[derive(Clone)]
pub struct GameObjectState {
    pub active: bool,
    pub parent_id: Option<i32>,
    pub child_ids: Vec<i32>,
}

impl GameObjectState {
    pub fn new(active: bool, parent_id: Option<i32>, child_ids: Vec<i32>) -> Self {
        Self { active, parent_id, child_ids }
    }

    pub fn parent(&self) -> Option<Arc<Mutex<GameObject>>> {
        self.parent_id.and_then(|id| GameObject::find_by_id(id))
    }

    pub fn children(&self) -> Vec<Arc<Mutex<GameObject>>> {
        self.child_ids.iter().filter_map(|&id| GameObject::find_by_id(id)).collect()
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn add_child(&mut self, child_id: i32) {
        if !self.child_ids.contains(&child_id) {
            self.child_ids.push(child_id);
        }
    }

    pub fn set_parent(&mut self, parent_id: i32) {
        self.parent_id = Some(parent_id);
    }
}

#[derive(Clone)]
pub struct GameObject {
    pub name: String,
    pub id: i32,
    pub components: Vec<Arc<Mutex<dyn component::ComponentTrait>>>,
    pub state: GameObjectState,
}

impl GameObject {
    pub fn new(name: String, components: Vec<Arc<Mutex<dyn component::ComponentTrait>>>, state: GameObjectState) -> Arc<Mutex<Self>> {
        let id = GAME_OBJECT_COUNT.lock().unwrap().get();
        // let id = *_id;
        let game_object = Arc::new(Mutex::new(Self { name, id, components, state }));
        GAME_OBJECT_REGISTRY.lock().unwrap().insert(id, game_object.clone());
        GAME_OBJECT_COUNT.lock().unwrap().increment();
        game_object
    }

    pub fn find_by_id(id: i32) -> Option<Arc<Mutex<Self>>> {
        GAME_OBJECT_REGISTRY.lock().unwrap().get(&id).cloned()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn update_name(&mut self, new: String) {
        self.name = new.to_string();
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn components(&self) -> &[Arc<Mutex<dyn component::ComponentTrait>>] {
        &self.components
    }

    pub fn add_component(&mut self, component: Arc<Mutex<dyn component::ComponentTrait>>) {
        self.components.push(component);
    }

    pub fn tick_self(&self) {
        for component in &self.components {
            let mut comp = component.lock().unwrap();
            comp.tick();
        }
    }

    pub fn tick_children(&self) {
        let children = self.state.children();
        for child_arc in children {
            let child = child_arc.lock().unwrap();
            child.tick_all();
        }
    }

    pub fn tick_all(&self) {
        self.tick_self();
        self.tick_children();
    }
}

pub fn make_base_game_object(name: String) -> Arc<Mutex<GameObject>> {
    GameObject::new(
        name,
        vec![],
        GameObjectState::new(
            true,
            None,
            vec![]
        )
    )
}

pub fn reparent(parent_id: i32, child_id: i32) {
    let mut registry = GAME_OBJECT_REGISTRY.lock().unwrap();
    if let (Some(parent_arc), Some(child_arc)) = (registry.get(&parent_id), registry.get(&child_id)) {
        let mut parent = parent_arc.lock().unwrap();
        let mut child = child_arc.lock().unwrap();
        child.state.set_parent(parent_id);
        parent.state.add_child(child_id);
    }
}

pub fn safe_to_object<F, T>(object: Arc<Mutex<GameObject>>, f: F) -> T
where
    F: FnOnce(&mut GameObject) -> T, 
{
    let mut game_object = object.lock().unwrap();  
    f(&mut *game_object)  
}

pub fn to_object<F, T>(object: i32, f: F) -> T
where
    F: FnOnce(&mut GameObject) -> T, 
{
    let mut game_object = GameObject::find_by_id(object).expect("Nothing found");
    let mut g = game_object.lock().unwrap();
    f(&mut g)  
}

pub fn _internal_to_object<T, F: FnOnce(&GameObject) -> T>(obj_id: i32, func: F) -> Option<T> {
    let game_objects = GAME_OBJECT_REGISTRY.lock().unwrap();
    if let Some(obj) = game_objects.get(&obj_id) {
        let obj = obj.lock().unwrap();
        return Some(func(&*obj));
    }
    None
}



pub fn add_component(object: i32, comp: Arc<Mutex<dyn component::ComponentTrait>>) {
    to_object(object, |obj| {
        obj.add_component(comp);
    })
}


pub fn has_component<T: component::ComponentTrait + 'static>(obj_id: i32) -> bool {
    let game_objects_lock = GAME_OBJECT_REGISTRY.lock().unwrap();  // Lock the registry
    if let Some(obj_arc) = game_objects_lock.get(&obj_id) {
        let game_object_lock = obj_arc.lock().unwrap();  // Lock the GameObject
        return game_object_lock.components().iter().any(|comp_mutex| {
            let comp = comp_mutex.lock().unwrap();  // Lock the component
            comp.type_id() == TypeId::of::<T>()  // Check the type ID
        });
    }
    false  // Return false if the GameObject was not found
}
