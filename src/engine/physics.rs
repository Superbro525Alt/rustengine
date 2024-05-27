use crate::engine::collider::{Collider, Point};
use crate::engine::component::{Rigidbody, Transform};
use crate::engine::gameobject::{colliding_point, colliding_with, GameObject};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct PhysicsEngine {
    pub game_objects: Vec<i32>,
    pub friction: f32,
}

impl PhysicsEngine {
    pub fn new(friction: f32) -> Self {
        Self {
            game_objects: Vec::new(),
            friction,
        }
    }

    pub fn add_object(&mut self, obj: i32) {
        self.game_objects.push(obj);
    }

    pub fn tick(&mut self, dt: f32) {
        // const GRAVITY: f32 = -9.8; // Gravity constant
        //
        // for &obj_id in &self.game_objects {
        //     let mut friction = None;
        //
        //     let mut g = GameObject::find_by_id(obj_id).expect("ok");
        //     let mut g = g.lock().unwrap();
        //     if g.get_component_closure::<Rigidbody>(|body| {
        //         friction = Some(body.friction);
        //     }) == None
        //     {
        //         continue;
        //     }
        //
        //     let mut effected_by_gravity: bool = false;
        //     let mut effected_by_collisions: bool = false;
        //
        //     g.get_component_closure::<Rigidbody>(|body| {
        //         effected_by_gravity = body.gravity;
        //         effected_by_collisions = body.collisions;
        //     });
        //
        //     if !(effected_by_collisions || effected_by_collisions) {
        //         continue;
        //     }
        //
        //     drop(g);
        //
        //     if let Some(game_object) = GameObject::find_by_id(obj_id) {
        //         let mut game_object = game_object.lock().unwrap();
        //
        //         // Apply gravity and friction to each GameObject
        //         if effected_by_gravity {
        //             game_object.get_component_closure::<Transform>(|transform| {
        //                 // Update position based on gravity
        //                 transform.inner[1] += GRAVITY * dt;
        //
        //                 // Apply friction to reduce velocity
        //                 transform.inner[0] *= 1.0 - friction.expect("ok") * dt;
        //                 transform.inner[2] *= 1.0 - friction.expect("ok") * dt;
        //             });
        //         }
        //
        //         // Check for collisions with other objects
        //         if effected_by_collisions {
        //             for &other_id in &self.game_objects {
        //                 if obj_id != other_id {
        //                     if let Some(mut other_game_object) = GameObject::find_by_id(other_id) {
        //                         let mut colls_lock = other_game_object.lock().unwrap();
        //                         let mut colls = colls_lock.colliders.clone();
        //                         drop(colls_lock);
        //                         for other_collider in colls.iter_mut() {
        //                             let mut other_pos: Option<Point> = None;
        //                             other_game_object
        //                                 .lock()
        //                                 .unwrap()
        //                                 .get_component_closure::<Transform>(|other_transform| {
        //                                     other_pos = Some(Point {
        //                                         x: other_transform.inner[0],
        //                                         y: other_transform.inner[1],
        //                                         z: other_transform.inner[2],
        //                                     });
        //                                 });
        //
        //                             if game_object.colliding_with(
        //                                 other_collider.clone(),
        //                                 other_pos.expect("nahh"),
        //                             ) {
        //                                 self.resolve_collision(
        //                                     &mut game_object,
        //                                     other_game_object.clone(),
        //                                 );
        //                                 println!(
        //                                     "Collision detected between {} and {}",
        //                                     obj_id, other_id
        //                                 );
        //                                 // thread::sleep(Duration::from_secs(1000));
        //                             }
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn resolve_collision(&self, obj1: &mut GameObject, obj2: Arc<Mutex<GameObject>>) {
        // obj1.get_component_closure::<Transform>(|transform1| {
        //     obj2.lock()
        //         .unwrap()
        //         .get_component_closure::<Transform>(|transform2| {
        //             let overlap = (transform1.inner[1] - transform2.inner[1]).abs();
        //             if overlap > 0.0 {
        //                 transform1.inner[1] += overlap / 2.0;
        //                 transform2.inner[1] -= overlap / 2.0;
        //             }
        //         });
        // });
    }
}
