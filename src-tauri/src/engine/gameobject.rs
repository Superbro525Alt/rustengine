use crate::engine::collider;
use crate::engine::collider::Collider;
use crate::engine::component;
use crate::engine::component::{ComponentTrait, TickBehavior, Transform};
use crate::engine::state::Engine;
use downcast_rs::Downcast;
use lazy_static::lazy_static;
use rocket::form::validate::Contains;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::collider::Point;

pub type MutexdGameObject = Arc<Mutex<GameObject>>;

lazy_static! {
    pub static ref GAME_OBJECT_REGISTRY: Mutex<HashMap<i32, Arc<Mutex<GameObject>>>> =
        Mutex::new(HashMap::new());
    static ref GAME_OBJECT_COUNT: Mutex<Count> = Mutex::new(Count::new());
    pub static ref GAME_OBJECT_DESTROYED: Mutex<Vec<i32>> = Mutex::new(Vec::new());
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

    pub fn child_ids(&self) -> Vec<i32> {
        self.child_ids.clone()
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
    pub colliders: Vec<Arc<Mutex<Box<dyn Collider>>>>,
    pub state: GameObjectState,
    pub render_references: Vec<usize>,
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
            render_references: Vec::new(),
            colliders: Vec::new(),
        }));
        GAME_OBJECT_REGISTRY
            .try_lock()
            .unwrap()
            .insert(id, game_object.clone());
        GAME_OBJECT_COUNT.lock().unwrap().increment();
        game_object
    }

    pub fn find_by_id(id: i32) -> Option<Arc<Mutex<Self>>> {
        if GAME_OBJECT_DESTROYED.lock().unwrap().contains(id) {
            return None;
        }

        let obj = GAME_OBJECT_REGISTRY.lock().unwrap().get(&id).cloned();

        if obj.is_some() {
            obj
        } else {
            let text = format!("No object with id {}", id);
            eprintln!("ERROR: {}", text);
            None
            // exit(1);
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

    pub fn colliders(&self) -> &[Arc<Mutex<Box<dyn Collider>>>] {
        &self.colliders
    }

    pub fn add_collider(&mut self, coll: Arc<Mutex<Box<dyn Collider>>>) {
        self.colliders.push(coll);
    }

    pub fn colliding_with(
        &mut self,
        other: Arc<Mutex<Box<dyn Collider>>>,
        other_pos: collider::Point,
    ) -> bool {
        let mut current_pos: [f32; 3] = [0.0, 0.0, 0.0];
        self.get_component_closure::<Transform>(|transform| {
            let current_pos = transform.pos.clone();
        });

        for coll in self.colliders.iter_mut() {
            if coll.lock().unwrap().colliding_with(
                &collider::Point {
                    x: current_pos[0],
                    y: current_pos[1],
                    z: current_pos[2],
                },
                other.clone(),
                &other_pos,
            ) {
                return true;
            }
        }

        false
    }

    pub fn colliding_point(&mut self, other: collider::Point) -> bool {
        let mut current_pos: [f32; 3] = [0.0, 0.0, 0.0];
        self.get_component_closure::<Transform>(|transform| {
            let current_pos = transform.pos.clone();
        });

        for coll in self.colliders.iter_mut() {
            if coll.lock().unwrap().colliding_point(
                &collider::Point {
                    x: current_pos[0],
                    y: current_pos[1],
                    z: current_pos[2],
                },
                &other,
            ) {
                return true;
            }
        }

        false
    }

    pub fn intersects(&mut self, segment: &mut (Point, Point)) -> bool {
        let mut current_pos: [f32; 3] = [0.0, 0.0, 0.0];
        self.get_component_closure::<Transform>(|transform| {
            current_pos = transform.pos.clone();
        });

        for coll in self.colliders.iter_mut() {
            if coll.try_lock().unwrap().intersects(
                segment,
                &Point {
                    x: current_pos[0],
                    y: current_pos[1],
                    z: current_pos[2]
                }
            ) {
                return true;
            }
        }

        false
    }

    pub fn get_component_closure<T>(&mut self, mut f: impl FnMut(&mut T)) -> Option<()>
    where
        T: ComponentTrait + 'static,
    {
        // Iterate over components
        let comps = self.components.clone();
        for comp_arc in comps {
            let mut _comp_lock = comp_arc.try_lock()/*.ok()?*/;

            let mut comp_lock = match _comp_lock {
                Ok(lock) => lock,
                Err(_) => {
                    println!("couldn't get lock");
                    continue;
                }
            };

            // println!("could get lock");

            // Attempt to downcast the component under the MutexGuard's scope
            // println!("{}", comp_lock.component);
            if let Some(component) = (&mut *comp_lock.component.lock().unwrap())
                .as_any_mut()
                .downcast_mut::<T>()
            {
                f(component); // Execute the closure with the mutable reference
                              // drop(comp_lock);
                return Some(()); // Return early on success
            }

            drop(comp_lock);
        }

        // If no component of type T was found and processed
        None
    }

    pub fn has_component<T: ComponentTrait + 'static>(&self) -> bool {
        for comp_arc in self.components.clone() {
            let comp_lock = comp_arc.lock().unwrap();
            if (&*comp_lock.component.lock().unwrap()).as_any().downcast_ref::<T>().is_some() {
                return true;
            }
        }
        false
    }

    pub fn tick_self(&mut self, engine: &mut Engine) {
        for component in self.components.clone() {
            // println!("{}", component.lock().unwrap().component.lock().unwrap().name().clone());
            let mut comp = component.lock().unwrap();

            let mut render_data = comp.tick(
                Some(&engine.input_data()),
                self,
                engine.dt.unwrap_or(Duration::from_secs(0)),
                engine.renderer.lock().unwrap().backend.camera.clone()
            );

            drop(comp);

            if render_data.is_some() {
                if self.state.active {
                    // if self.render_reference.is_some() {
                    //     engine.remove_from_render_queue(
                    //         self.render_reference.expect("no render reference"),
                    //     );
                    // }

                    self.render_references.push(engine.render(render_data.take().expect("get good")));
                } else {
                    // engine.remove_from_render_queue(
                    //     self.render_reference.expect("no render reference"),
                    // );
                }
            }

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

    pub fn destroy(&mut self) {
        GAME_OBJECT_DESTROYED.lock().unwrap().push(self.id); 
    } 
}

pub fn make_base_game_object(name: String) -> Arc<Mutex<GameObject>> {
    let g = GameObject::new(name, vec![], GameObjectState::new(true, None, vec![]));

    let id = g.clone().lock().unwrap().id().clone();
    add_component(id, component::Transform::new());

    g
}

pub fn colliding_with(
    obj_id: i32,
    other: Arc<Mutex<Box<dyn Collider>>>,
    other_pos: collider::Point,
) -> bool {
    let mut obj_op = GameObject::find_by_id(obj_id);
    let mut obj = obj_op.expect("no object");

    let mut lock = obj.lock().unwrap();

    lock.colliding_with(other, other_pos)
}

pub fn colliding_point(obj_id: i32, other: collider::Point) -> bool {
    let mut obj_op = GameObject::find_by_id(obj_id);
    let mut obj = obj_op.expect("no object");

    let mut lock = obj.lock().unwrap();

    lock.colliding_point(other)
}

pub fn add_collider(obj_id: i32, coll: Arc<Mutex<Box<dyn Collider>>>) {
    let mut obj_op = GameObject::find_by_id(obj_id);
    let obj = obj_op.expect("no object");

    let mut lock = obj.lock().unwrap();

    lock.add_collider(coll);
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
    let mut g = game_object.try_lock().unwrap();
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
    let comp_type_id = comp.type_id();

    let game_objects = GAME_OBJECT_REGISTRY
        .try_lock()
        .expect("Registry lock failed");
    let game_object = match game_objects.get(&object) {
        Some(obj_arc) => obj_arc.lock().expect("GameObject lock failed"),
        None => {
            eprintln!("ERROR: No object with id {}", object);
            return;
        }
    };

    let already_exists = game_object.components.iter().any(|comp_arc| {
        let comp = comp_arc.lock().unwrap();
        TypeId::of::<dyn ComponentTrait>() == comp_type_id
    });

    drop(game_object);
    drop(game_objects);

    if already_exists {
        let game_object_name = GameObject::find_by_id(object)
            .unwrap()
            .try_lock()
            .unwrap()
            .name()
            .to_string();
        eprintln!(
            "ERROR: GameObject {} already has component {}",
            game_object_name,
            comp.lock().unwrap().component.lock().unwrap().name()
        );
        exit(1);
    } else {
        to_object(object, |obj| {
            obj.add_component(comp);
        });
    }
}

pub fn has_component<T: component::ComponentTrait + 'static>(obj_id: i32) -> bool {
    let game_object = GameObject::find_by_id(obj_id).expect("No object found");
    let lock = game_object.lock().unwrap();
    lock.has_component::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::collider::Point;
    use crate::engine::component::{ComponentWrapper, Transform};
    use crate::engine::components::{RenderComponent, InputComponent};
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_game_object_creation() {
        let name = "TestObject".to_string();
        let components = vec![];
        let state = GameObjectState::new(true, None, vec![]);
        let game_object = GameObject::new(name.clone(), components, state);

        let game_object = game_object.lock().unwrap();
        assert_eq!(game_object.name(), name);
        assert_eq!(game_object.state.active(), true);
    }

    #[test]
    fn test_add_component() {
        let name = "TestObject".to_string();
        let game_object = make_base_game_object(name);

        // let id = game_object.lock().unwrap().id();
        let transform_component = InputComponent::new(String::from("name"));

        // add_component(id, transform_component.clone());
        game_object.lock().unwrap().add_component(transform_component.clone());
        assert!(game_object.lock().unwrap().has_component::<InputComponent>());

        let mut has = false;

        game_object.lock().unwrap().get_component_closure::<InputComponent>(|input| {
            has = true;
        });
        assert!(has);
    }

    // #[test]
    // fn test_add_collider() {
    //     let name = "TestObject".to_string();
    //     let game_object = make_base_game_object(name);
    //
    //     let id = game_object.lock().unwrap().id();
    //     let collider = Arc::new(Mutex::new(collider::PointCollider::new(Point { x: 0.0, y: 0.0, z: 0.0 })));
    //
    //     add_collider(id, collider.clone());
    //     assert!(game_object.lock().unwrap().colliders().contains(&collider));
    // }

    #[test]
    fn test_reparent() {
        let parent_name = "ParentObject".to_string();
        let child_name = "ChildObject".to_string();

        let parent_object = make_base_game_object(parent_name);
        let child_object = make_base_game_object(child_name);

        let parent_id = parent_object.lock().unwrap().id();
        let child_id = child_object.lock().unwrap().id();

        reparent(parent_id, child_id);

        let child_object = child_object.lock().unwrap();
        assert_eq!(child_object.state.parent_id, Some(parent_id));

        let parent_object = parent_object.lock().unwrap();
        assert!(parent_object.state.child_ids.contains(&child_id));
    }

    #[test]
    fn test_colliding_with() {
        let name1 = "Object1".to_string();
        let name2 = "Object2".to_string();

        let object1 = make_base_game_object(name1);
        let object2 = make_base_game_object(name2);

        let id1 = object1.lock().unwrap().id();
        let id2 = object2.lock().unwrap().id();

        let collider1 = Arc::new(Mutex::new(collider::PointCollider::new(Point { x: 0.0, y: 0.0, z: 0.0 })));
        let collider2 = Arc::new(Mutex::new(collider::PointCollider::new(Point { x: 0.0, y: 0.0, z: 0.0 })));

        add_collider(id1, collider1.clone());
        add_collider(id2, collider2.clone());

        assert!(colliding_with(id1, collider2, Point { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_colliding_point() {
        let name = "Object".to_string();
        let object = make_base_game_object(name);

        let id = object.lock().unwrap().id();
        let collider = Arc::new(Mutex::new(collider::PointCollider::new(Point { x: 0.0, y: 0.0, z: 0.0 })));

        add_collider(id, collider.clone());
        assert!(colliding_point(id, Point { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_update_name() {
        let name = "TestObject".to_string();
        let new_name = "UpdatedObject".to_string();
        let game_object = make_base_game_object(name);

        game_object.lock().unwrap().update_name(new_name.clone());
        assert_eq!(game_object.lock().unwrap().name(), &new_name);
    }
}
