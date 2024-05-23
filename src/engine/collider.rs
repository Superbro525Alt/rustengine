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
    fn points(&self) -> Vec<Point>;

    fn colliding(&self, current_pos: &Point, other_bounds: &Point) -> bool {
        let self_points = self.points();

        let self_points: Vec<Point> = self_points
            .iter()
            .map(|p| Point {
                x: p.x + current_pos.x,
                y: p.y + current_pos.y,
                z: p.z + current_pos.z,
            })
            .collect();

        let self_bounds = calculate_aabb(&self_points);

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

        aabb_collision(&self_bounds, &other_min, &other_max)
    }

    fn colliding_with(
        &self,
        current_pos: &Point,
        other: Arc<Mutex<dyn Collider>>,
        other_pos: &Point,
    ) -> bool {
        let self_points = self.points();
        let other_points = other.lock().unwrap().points();

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

        let self_bounds = calculate_aabb(&self_points);
        let other_bounds = calculate_aabb(&other_points);

        aabb_collision(&self_bounds, &other_bounds.0, &other_bounds.1)
    }

    fn colliding_point(&self, current_pos: &Point, point: &Point) -> bool {
        let self_points = self.points();

        let self_points: Vec<Point> = self_points
            .iter()
            .map(|p| Point {
                x: p.x + current_pos.x,
                y: p.y + current_pos.y,
                z: p.z + current_pos.z,
            })
            .collect();

        let bounds = calculate_aabb(&self_points);

        point_inside_aabb(point, &bounds.0, &bounds.1)
    }
}

fn calculate_aabb(points: &[Point]) -> (Point, Point) {
    let mut min = Point {
        x: f32::MAX,
        y: f32::MAX,
        z: f32::MAX,
    };
    let mut max = Point {
        x: f32::MIN,
        y: f32::MIN,
        z: f32::MIN,
    };

    for point in points {
        if point.x < min.x {
            min.x = point.x;
        }
        if point.y < min.y {
            min.y = point.y;
        }
        if point.z < min.z {
            min.z = point.z;
        }
        if point.x > max.x {
            max.x = point.x;
        }
        if point.y > max.y {
            max.y = point.y;
        }
        if point.z > max.z {
            max.z = point.z;
        }
    }

    (min, max)
}

fn aabb_collision(aabb1: &(Point, Point), aabb2_min: &Point, aabb2_max: &Point) -> bool {
    !(aabb1.1.x < aabb2_min.x
        || aabb1.0.x > aabb2_max.x
        || aabb1.1.y < aabb2_min.y
        || aabb1.0.y > aabb2_max.y
        || aabb1.1.z < aabb2_min.z
        || aabb1.0.z > aabb2_max.z)
}

fn point_inside_aabb(point: &Point, min: &Point, max: &Point) -> bool {
    point.x >= min.x
        && point.x <= max.x
        && point.y >= min.y
        && point.y <= max.y
        && point.z >= min.z
        && point.z <= max.z
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
    fn points(&self) -> Vec<Point> {
        let half_side = self.side_length / 2.0;
        vec![
            Point {
                x: -half_side,
                y: -half_side,
                z: -half_side,
            },
            Point {
                x: half_side,
                y: -half_side,
                z: -half_side,
            },
            Point {
                x: half_side,
                y: half_side,
                z: -half_side,
            },
            Point {
                x: -half_side,
                y: half_side,
                z: -half_side,
            },
            Point {
                x: -half_side,
                y: -half_side,
                z: half_side,
            },
            Point {
                x: half_side,
                y: -half_side,
                z: half_side,
            },
            Point {
                x: half_side,
                y: half_side,
                z: half_side,
            },
            Point {
                x: -half_side,
                y: half_side,
                z: half_side,
            },
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
        Self {
            width,
            height,
            depth,
        }
    }
}

impl Collider for RectangularPrismCollider {
    fn points(&self) -> Vec<Point> {
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let half_depth = self.depth / 2.0;
        vec![
            Point {
                x: -half_width,
                y: -half_height,
                z: -half_depth,
            },
            Point {
                x: half_width,
                y: -half_height,
                z: -half_depth,
            },
            Point {
                x: half_width,
                y: half_height,
                z: -half_depth,
            },
            Point {
                x: -half_width,
                y: half_height,
                z: -half_depth,
            },
            Point {
                x: -half_width,
                y: -half_height,
                z: half_depth,
            },
            Point {
                x: half_width,
                y: -half_height,
                z: half_depth,
            },
            Point {
                x: half_width,
                y: half_height,
                z: half_depth,
            },
            Point {
                x: -half_width,
                y: half_height,
                z: half_depth,
            },
        ]
    }
}
