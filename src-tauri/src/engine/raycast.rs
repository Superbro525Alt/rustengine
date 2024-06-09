use std::{sync::{Arc, Mutex}, f32::consts::PI};

use super::{gameobject::{GameObject, self, GAME_OBJECT_REGISTRY}, collider::Point, collider, state::Engine, graphics_backend::primitives::{Primitives, Line, RaycastLine}};

#[derive(Debug)]
pub struct CastError {

}

pub struct RaycastResult {
    pub underlying: Vec<Arc<Mutex<GameObject>>>    
}

impl RaycastResult {
    pub fn new() -> Self {
        Self {
            underlying: Vec::new()
        }
    }

    pub fn add(&mut self, obj: Arc<Mutex<GameObject>>) {
        self.underlying.push(obj);
    }
}

pub struct Raycast {

}

impl Raycast {
    pub fn show(initial_position: Point, angle: f32, length: f32, thickness: f32) -> RaycastLine {
        RaycastLine::new(initial_position, angle, length, thickness, [1.0, 0.0, 0.0])
        // // println!("angle: {}", angle);
        // Line::new(initial_position, angle*(PI/180.0), length, thickness, [1.0, 0.0, 0.0])
    }

    pub fn send(initial_position: Point, mut angle: f32, length: f32, ignore: Vec<i32>) -> Result<RaycastResult, CastError> {
        let mut res = RaycastResult::new();

        angle *= (PI/180.0); // convert to radians
        
        let direction = Point {
            x: (angle).cos(),
            y: (angle).sin(),
            z: 0.0, 
        };
        
        for obj in gameobject::GAME_OBJECT_REGISTRY.try_lock().unwrap().iter() {
            if gameobject::GAME_OBJECT_DESTROYED.lock().unwrap().contains(obj.0) {
                continue;
            }


            if !ignore.contains(obj.0) {
                let mut game_object = obj.1.try_lock().unwrap();

                let mut line_segment = (initial_position.clone(), initial_position.clone() + (direction.clone() * length));

                if game_object.intersects(&mut line_segment) {
                    res.add(obj.1.clone());
                }
            }
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gameobject::{GameObject, GameObjectState};
    use crate::engine::collider::{Point, Collider};
    use std::sync::{Arc, Mutex};
    use serial_test::serial;

    struct MockCollider {
        points: Vec<Point>,
    }

    impl MockCollider {
        pub fn new(points: Vec<Point>) -> Box<dyn Collider> {
            Box::new(Self { points })
        }
    }

    impl collider::Collider for MockCollider {
        fn points(&self) -> Vec<Point> {
            self.points.clone()
        }
    }

    #[serial]
    fn test_raycast_no_collision() {
        let initial_position = Point { x: 0.0, y: 0.0, z: 0.0 };
        let angle = 0.0;
        let length = 10.0;
        let ignore = vec![];

        let result = Raycast::send(initial_position, angle, length, ignore).unwrap();
        assert_eq!(result.underlying.len(), 0);
    }

    #[serial]
    fn test_raycast_single_collision() {
        let initial_position = Point { x: 0.0, y: 0.0, z: 0.0 };
        let angle = 0.0;
        let length = 10.0;
        let ignore = vec![];

        let game_object = gameobject::make_base_game_object(
            "TestObject".to_string(),
        );
        game_object.lock().unwrap().add_collider(Arc::new(Mutex::new(MockCollider::new(
            vec![
                Point { x: 5.0, y: -1.0, z: 0.0 },
                Point { x: 5.0, y: 1.0, z: 0.0 },
                Point { x: 6.0, y: 1.0, z: 0.0 },
                Point { x: 6.0, y: -1.0, z: 0.0 },
            ],
        ))));

        let result = Raycast::send(initial_position, angle, length, ignore).unwrap();
        assert_eq!(result.underlying.len(), 1);

        game_object.lock().unwrap().destroy();
    }

    #[serial]
    fn test_raycast_multiple_collisions() {
        let initial_position = Point { x: 0.0, y: 0.0, z: 0.0 };
        let angle = 0.0;
        let length = 10.0;
        let ignore = vec![];

        let game_object1 = gameobject::make_base_game_object(
            "TestObject1".to_string(),
        );
        game_object1.lock().unwrap().add_collider(Arc::new(Mutex::new(MockCollider::new(
            vec![
                Point { x: 3.0, y: -1.0, z: 0.0 },
                Point { x: 3.0, y: 1.0, z: 0.0 },
                Point { x: 4.0, y: 1.0, z: 0.0 },
                Point { x: 4.0, y: -1.0, z: 0.0 },
            ],
        ))));

        let game_object2 = gameobject::make_base_game_object(
            "TestObject2".to_string(),
        );
        game_object2.lock().unwrap().add_collider(Arc::new(Mutex::new(MockCollider::new(
            vec![
                Point { x: 7.0, y: -1.0, z: 0.0 },
                Point { x: 7.0, y: 1.0, z: 0.0 },
                Point { x: 8.0, y: 1.0, z: 0.0 },
                Point { x: 8.0, y: -1.0, z: 0.0 },
            ],
        ))));

        let result = Raycast::send(initial_position, angle, length, ignore).unwrap();
        assert_eq!(result.underlying.len(), 2);
        game_object1.lock().unwrap().destroy();
        game_object2.lock().unwrap().destroy();
    }

    #[serial]
    fn test_raycast_ignore_collision() {
        let initial_position = Point { x: 0.0, y: 0.0, z: 0.0 };
        let angle = 0.0;
        let length = 10.0;
        let ignore = vec![1];

        let game_object1 = gameobject::make_base_game_object(
            "TestObject1".to_string(),
        );

        game_object1.lock().unwrap().add_collider(Arc::new(Mutex::new(MockCollider::new(
            vec![
                Point { x: 3.0, y: -1.0, z: 0.0 },
                Point { x: 3.0, y: 1.0, z: 0.0 },
                Point { x: 4.0, y: 1.0, z: 0.0 },
                Point { x: 4.0, y: -1.0, z: 0.0 },
            ],
        ))));

        let game_object2 = gameobject::make_base_game_object(
            "TestObject2".to_string(),
        );
        game_object2.lock().unwrap().add_collider(Arc::new(Mutex::new(MockCollider::new(
            vec![
                Point { x: 7.0, y: -1.0, z: 0.0 },
                Point { x: 7.0, y: 1.0, z: 0.0 },
                Point { x: 8.0, y: 1.0, z: 0.0 },
                Point { x: 8.0, y: -1.0, z: 0.0 },
            ],
        ))));

        let result = Raycast::send(initial_position, angle, length, ignore).unwrap();
        assert_eq!(result.underlying.len(), 1);
        game_object1.lock().unwrap().destroy();
        game_object2.lock().unwrap().destroy();
    }
}

