use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait Collider: Send + Sync
where
    Self: 'static,
{
    fn points(&mut self) -> Vec<Point>;

    fn colliding(&mut self, current_pos: &Point, other_bounds: &Point) -> bool {
        let self_points = self.points();

        let self_points: Vec<Point> = self_points
            .iter()
            .map(|p| Point {
                x: p.x + current_pos.x,
                y: p.y + current_pos.y,
                z: p.z + current_pos.z,
            })
            .collect();

        // Calculate the other collider points as AABB
        let other_min = Point {
            x: current_pos.x - other_bounds.x / 2.0,
            y: current_pos.y - other_bounds.y / 2.0,
            z: current_pos.z - other_bounds.z / 2.0,
        };
        let other_max = Point {
            x: current_pos.x + other_bounds.x / 2.0,
            y: current_pos.y + other_bounds.y / 2.0,
            z: current_pos.z + other_bounds.z / 2.0,
        };

        // Check for collision with AABB
        if sat_collision_with_aabb(&self_points, &other_min, &other_max) {
            return true;
        }

        false
    }

    fn colliding_with(&mut self, current_pos: &Point, other: Arc<Mutex<dyn Collider>>, other_pos: &Point) -> bool {
        let self_points = self.points();
        let other_points = other.lock().unwrap().points();

        // Translate points to their current positions
        let self_points: Vec<Point> = self_points
            .iter()
            .map(|p| Point {
                x: p.x + current_pos.x,
                y: p.y + current_pos.y,
                z: p.z + current_pos.z,
            })
            .collect();

        let other_points: Vec<Point> = other_points
            .iter()
            .map(|p| Point {
                x: p.x + other_pos.x,
                y: p.y + other_pos.y,
                z: p.z + other_pos.z,
            })
            .collect();

        // Check for collision using SAT
        if sat_collision(&self_points, &other_points) {
            return true;
        }

        false
    }

    fn colliding_point(&mut self, current_pos: &Point, point: &Point) -> bool {
        let self_points = self.points();

        // Translate self points to their current position
        let self_points: Vec<Point> = self_points
            .iter()
            .map(|p| Point {
                x: p.x + current_pos.x,
                y: p.y + current_pos.y,
                z: p.z + current_pos.z,
            })
            .collect();

        // Check if the point is within the bounds of the collider using SAT
        for axis in &["x", "y", "z"] {
            let (min_self, max_self) = project(&self_points, axis);
            let value = match *axis {
                "x" => point.x,
                "y" => point.y,
                "z" => point.z,
                _ => unreachable!(),
            };

            if value < min_self || value > max_self {
                return false;
            }
        }

        true
    }
}

fn sat_collision(points_a: &Vec<Point>, points_b: &Vec<Point>) -> bool {
    for axis in &["x", "y", "z"] {
        let (min_a, max_a) = project(points_a, axis);
        let (min_b, max_b) = project(points_b, axis);

        if max_a < min_b || min_a > max_b {
            return false;
        }
    }

    true
}

fn sat_collision_with_aabb(points: &Vec<Point>, other_min: &Point, other_max: &Point) -> bool {
    for axis in &["x", "y", "z"] {
        let (min_self, max_self) = project(points, axis);
        let (min_other, max_other) = match *axis {
            "x" => (other_min.x, other_max.x),
            "y" => (other_min.y, other_max.y),
            "z" => (other_min.z, other_max.z),
            _ => unreachable!(),
        };

        if max_self < min_other || min_self > max_other {
            return false;
        }
    }

    true
}

fn project(points: &Vec<Point>, axis: &str) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    for point in points {
        let value = match axis {
            "x" => point.x,
            "y" => point.y,
            "z" => point.z,
            _ => unreachable!(),
        };

        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }

    (min, max)
}

pub struct CubeCollider {
    side_length: f32,
}

impl CubeCollider {
    pub fn new(side_length: f32) -> Self {
        Self { side_length }
    }
}

impl Collider for CubeCollider {
    fn points(&mut self) -> Vec<Point> {
        let half_side = self.side_length / 2.0;
        vec![
            Point { x: -half_side, y: -half_side, z: -half_side },
            Point { x: half_side, y: -half_side, z: -half_side },
            Point { x: half_side, y: half_side, z: -half_side },
            Point { x: -half_side, y: half_side, z: -half_side },
            Point { x: -half_side, y: -half_side, z: half_side },
            Point { x: half_side, y: -half_side, z: half_side },
            Point { x: half_side, y: half_side, z: half_side },
            Point { x: -half_side, y: half_side, z: half_side },
        ]
    }
}

pub struct RectangularPrismCollider {
    width: f32,
    height: f32,
    depth: f32,
}

impl RectangularPrismCollider {
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        Self { width, height, depth }
    }
}

impl Collider for RectangularPrismCollider {
    fn points(&mut self) -> Vec<Point> {
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let half_depth = self.depth / 2.0;
        vec![
            Point { x: -half_width, y: -half_height, z: -half_depth },
            Point { x: half_width, y: -half_height, z: -half_depth },
            Point { x: half_width, y: half_height, z: -half_depth },
            Point { x: -half_width, y: half_height, z: -half_depth },
            Point { x: -half_width, y: -half_height, z: half_depth },
            Point { x: half_width, y: -half_height, z: half_depth },
            Point { x: half_width, y: half_height, z: half_depth },
            Point { x: -half_width, y: half_height, z: half_depth },
        ]
    }
}
