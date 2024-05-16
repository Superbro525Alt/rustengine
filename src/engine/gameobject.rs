use crate::engine::component;
use crate::engine::component::{ComponentTrait, TickBehavior};
use crate::engine::state::Engine;
use colored::Colorize;
use downcast_rs::Downcast;
use lazy_static::lazy_static;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::process::exit;
use std::sync::{Arc, Mutex};

pub type MutexdGameObject = Arc<Mutex<GameObject>>;

lazy_static! {
    pub static ref GAME_OBJECT_REGISTRY: Mutex<HashMap<i32, Arc<Mutex<GameObject>>>> =
        Mutex::new(HashMap::new());
    static ref GAME_OBJECT_COUNT: Mutex<Count> = Mutex::new(Count::new());
}

#[derive(Clone)]
struct Count {
    internal: i32,
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
        Self {
            active,
            parent_id,
            child_ids,
        }
    }

    pub fn parent(&self) -> Option<Arc<Mutex<GameObject>>> {
        self.parent_id.and_then(GameObject::find_by_id)
    }

    pub fn children(&self) -> Vec<Arc<Mutex<GameObject>>> {
        self.child_ids
            .iter()
            .filter_map(|&id| GameObject::find_by_id(id))
            .collect()
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
    pub components: Vec<Arc<Mutex<component::ComponentWrapper>>>,
    pub state: GameObjectState,
    render_reference: Option<usize>,
}

impl GameObject {
    pub fn new(
        name: String,
        components: Vec<Arc<Mutex<component::ComponentWrapper>>>,
        state: GameObjectState,
    ) -> Arc<Mutex<Self>> {
        let id = GAME_OBJECT_COUNT.lock().unwrap().get();
        // let id = *_id;
        let game_object = Arc::new(Mutex::new(Self {
            name,
            id,
            components,
            state,
            render_reference: None,
        }));
        GAME_OBJECT_REGISTRY
            .lock()
            .unwrap()
            .insert(id, game_object.clone());
        GAME_OBJECT_COUNT.lock().unwrap().increment();
        game_object
    }

    pub fn find_by_id(id: i32) -> Option<Arc<Mutex<Self>>> {
        let obj = GAME_OBJECT_REGISTRY.lock().unwrap().get(&id).cloned();

        if obj.is_some() {
            obj
        } else {
            let text = format!("No object with id {}", id);
            eprintln!("ERROR: {}", text);
            exit(1);
        }
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

    pub fn components(&self) -> &[Arc<Mutex<component::ComponentWrapper>>] {
        &self.components
    }

    pub fn add_component(&mut self, component: Arc<Mutex<component::ComponentWrapper>>) {
        self.components.push(component);
    }

    // pub fn get_component<T: component::ComponentTrait + 'static, F>(&self, mut f: F)
    // where
    //     F: FnMut(&mut T),
    // {
    //     self.components.iter().find_map(|comp| {
    //         let mut comp_lock = comp.lock().unwrap();
    //         if let Some(mut casted_comp) = comp_lock.as_any_mut().downcast_mut::<T>() {
    //             Some(f(&mut casted_comp))
    //         } else {
    //             None
    //         }
    //     });
    // }

    pub fn get_component<T: ComponentTrait + 'static>(
        &self,
    ) -> Option<Arc<Mutex<dyn component::ComponentTrait>>> {
        self.components.iter().find_map(|wrapper| {
            let wrapper = wrapper.lock().unwrap();
            if wrapper.component.as_any().downcast_ref::<T>().is_some() {
                Some(wrapper.component.clone())
            } else {
                None
            }
        })
    }

    pub fn input_data(&mut self) -> component::InputData {
        component::InputData {}
    }

    pub fn tick_self(&mut self, engine: &mut Engine) {
        for component in &self.components.clone() {
            let mut comp = component.lock().unwrap();
            let mut render_data = comp.tick(Some(&self.input_data()));
            println!(
                "render first step: {:?}",
                render_data.as_mut().expect("nahh").obj.desc_raw()
            );
            if render_data.is_some() {
                println!("renderdata is some");
                if self.state.active {
                    println!("active");
                    if self.render_reference.is_some() {
                        engine.remove_from_render_queue(
                            self.render_reference.expect("no render reference"),
                        );
                    }

                    println!("ahhh");

                    self.render_reference =
                        Some(engine.render(render_data.take().expect("get good")));
                    println!("rendering");
                } else {
                    engine.remove_from_render_queue(
                        self.render_reference.expect("no render reference"),
                    );
                }
            }

            println!("{:?}", self.render_reference);
            // match comp
            // comp.tick();
        }
    }

    pub fn tick_children(&mut self, engine: &mut Engine) {
        let children = self.state.children();
        for child_arc in children {
            let mut child = child_arc.lock().unwrap();
            child.tick_all(engine);
        }
    }

    pub fn tick_all(&mut self, engine: &mut Engine) {
        self.tick_self(engine);
        self.tick_children(engine);
    }
}

pub fn make_base_game_object(name: String) -> Arc<Mutex<GameObject>> {
    GameObject::new(name, vec![], GameObjectState::new(true, None, vec![]))
}

pub fn reparent(parent_id: i32, child_id: i32) {
    let registry = GAME_OBJECT_REGISTRY.lock().unwrap();
    if let (Some(parent_arc), Some(child_arc)) = (registry.get(&parent_id), registry.get(&child_id))
    {
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
    f(&mut game_object)
}

pub fn to_object<F, T>(object: i32, f: F) -> T
where
    F: FnOnce(&mut GameObject) -> T,
{
    let game_object = GameObject::find_by_id(object).expect("Nothing found");
    let mut g = game_object.lock().unwrap();
    f(&mut g)
}

pub fn _internal_to_object<T, F: FnOnce(&GameObject) -> T>(obj_id: i32, func: F) -> Option<T> {
    let game_objects = GAME_OBJECT_REGISTRY.lock().unwrap();
    if let Some(obj) = game_objects.get(&obj_id) {
        let obj = obj.lock().unwrap();
        return Some(func(&obj));
    }
    None
}

// pub fn add_component(object: i32, comp: Arc<Mutex<dyn component::TickVariant>>) {

pub fn add_component(object: i32, comp: Arc<Mutex<component::ComponentWrapper>>) {
    let comp_type_id = {
        let comp_lock = comp.lock().unwrap();
        let variant_lock = comp_lock.ticker.lock().unwrap();
        match &*variant_lock {
            component::TickVariant::Render(renderer) => {
                TypeId::of::<dyn component::RenderTickBehavior>()
            }
            component::TickVariant::Default(renderer) => {
                TypeId::of::<dyn component::RenderTickBehavior>()
            }
            component::TickVariant::Input(renderer) => {
                TypeId::of::<dyn component::RenderTickBehavior>()
            } // Add other match arms if other variants exist
        }
    };

    let already_exists = to_object(object, |obj| {
        obj.components().iter().any(|existing_comp| {
            let existing_comp_lock = existing_comp.lock().unwrap();
            let existing_variant_lock = existing_comp_lock.ticker.lock().unwrap();
            match &*existing_variant_lock {
                component::TickVariant::Render(_) => {
                    TypeId::of::<dyn component::RenderTickBehavior>() == comp_type_id
                }
                component::TickVariant::Default(_) => {
                    TypeId::of::<dyn component::RenderTickBehavior>() == comp_type_id
                }
                component::TickVariant::Input(_) => {
                    TypeId::of::<dyn component::RenderTickBehavior>() == comp_type_id
                } // Match other variants similarly
            }
        })
    });

    if already_exists {
        // let comp_name = comp.lock().unwrap().component.lock().unwrap().name();
        // let game_object_name = GameObject::find_by_id(object).unwrap().lock().unwrap().name().to_string();
        // eprintln!("ERROR: GameObject {} already has component {}", game_object_name, comp_name);
        exit(1);
    } else {
        to_object(object, |obj| {
            obj.add_component(comp);
        });
    }
}

pub fn has_component<T: component::ComponentTrait + 'static>(obj_id: i32) -> bool {
    let game_objects = GAME_OBJECT_REGISTRY.lock().expect("Registry lock failed");
    let game_object = match game_objects.get(&obj_id) {
        Some(obj_arc) => obj_arc.lock().expect("GameObject lock failed"),
        None => {
            eprintln!("ERROR: No object with id {}", obj_id);
            exit(1);
        }
    };

    let comp_type_id = TypeId::of::<T>();

    let already_exists = to_object(obj_id, |obj| {
        obj.components().iter().any(|existing_comp| {
            let existing_comp_lock = existing_comp.lock().unwrap();
            let existing_variant_lock = existing_comp_lock.ticker.lock().unwrap();
            match &*existing_variant_lock {
                component::TickVariant::Render(_) => {
                    TypeId::of::<dyn component::RenderTickBehavior>() == comp_type_id
                }
                component::TickVariant::Default(_) => {
                    TypeId::of::<dyn component::RenderTickBehavior>() == comp_type_id
                }
                component::TickVariant::Input(_) => {
                    TypeId::of::<dyn component::RenderTickBehavior>() == comp_type_id
                } // Match other variants similarly
            }
        })
    });

    already_exists
}

pub fn get_component<T: component::ComponentTrait + 'static>(obj_id: i32) {
    if has_component::<T>(obj_id) {
        to_object(obj_id, |obj| {
            obj.get_component::<T>();
        })
    } else {
        eprintln!(
            "ERROR: The object with id {} does not contain the requested component",
            obj_id
        );
        exit(1);
    }
}
