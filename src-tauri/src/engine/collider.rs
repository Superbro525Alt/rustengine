use std::sync::{Arc, Mutex};
use std::ops::{Mul, Add};
use downcast_rs::{impl_downcast, Downcast};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Point {}

pub trait Collider: Send + Sync + Downcast
where
    Self: 'static,
{
    fn points(&self) -> Vec<Point>;

    fn intersects(&self, line_segment: &(Point, Point), current_pos: &Point) -> bool {
        let mut points = self.points();
        translate_points(&mut points, current_pos);

        // println!("{:?} {:?}", points, current_pos);

        for i in 0..points.len() {
            let p1 = &points[i];
            let p2 = &points[(i + 1) % points.len()];
            if line_segment_intersects(&line_segment.0, &line_segment.1, p1, p2) {
                return true;
            }
        }

        self.colliding_point(current_pos, &line_segment.0) || 
        self.colliding_point(current_pos, &line_segment.1)
    }

    fn colliding_with(
        &self,
        current_pos: &Point,
        other: Arc<Mutex<Box<dyn Collider>>>,
        other_pos: &Point,
    ) -> bool {
        let mut self_points = self.points();
        translate_points(&mut self_points, current_pos);

        let mut other_points = other.lock().unwrap().points();
        translate_points(&mut other_points, other_pos);

        let self_bounds = calculate_aabb(&self_points);
        let other_bounds = calculate_aabb(&other_points);

        aabb_collision(&self_bounds, &other_bounds)
    }

    fn colliding_point(&self, current_pos: &Point, point: &Point) -> bool {
        let mut self_points = self.points();
        translate_points(&mut self_points, current_pos);

        let bounds = calculate_aabb(&self_points);
        point_inside_aabb(point, &bounds.0, &bounds.1) || 
        (self_points.len() == 1 && *point == self_points[0])
    }

    // fn as_any(&self) -> &dyn std::any::Any where Self: Sized { self }
}

impl_downcast!(Collider);

fn translate_points(points: &mut [Point], translation: &Point) {
    for point in points.iter_mut() {
        point.x += translation.x;
        point.y += translation.y;
        point.z += translation.z;
    }
}

fn line_segment_intersects(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
    let o1 = orientation(p1, p2, p3);
    let o2 = orientation(p1, p2, p4);
    let o3 = orientation(p3, p4, p1);
    let o4 = orientation(p3, p4, p2);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special cases
    // p1, p2, and p3 are collinear and p3 lies on segment p1p2
    if o1 == 0 && on_segment(p1, p2, p3) {
        return true;
    }
    // p1, p2, and p4 are collinear and p4 lies on segment p1p2
    if o2 == 0 && on_segment(p1, p2, p4) {
        return true;
    }
    // p3, p4, and p1 are collinear and p1 lies on segment p3p4
    if o3 == 0 && on_segment(p3, p4, p1) {
        return true;
    }
    // p3, p4, and p2 are collinear and p2 lies on segment p3p4
    if o4 == 0 && on_segment(p3, p4, p2) {
        return true;
    }

    // Doesn't fall in any of the above cases
    false
}

fn orientation(p: &Point, q: &Point, r: &Point) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val.abs() < std::f32::EPSILON {
        0
    } else {
        val.signum() as i32
    }
}

fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {
    r.x <= p.x.max(q.x) && r.x >= p.x.min(q.x) && 
    r.y <= p.y.max(q.y) && r.y >= p.y.min(q.y) &&
    r.z <= p.z.max(q.z) && r.z >= p.z.min(q.z)
}

fn calculate_aabb(points: &[Point]) -> (Point, Point) {
    let (mut min, mut max) = (points[0].clone(), points[0].clone());
    for point in points.iter().skip(1) {
        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        min.z = min.z.min(point.z);
        max.x = max.x.max(point.x);
        max.y = max.y.max(point.y);
        max.z = max.z.max(point.z);
    }
    (min, max)
}

fn aabb_collision(aabb1: &(Point, Point), aabb2: &(Point, Point)) -> bool {
    !(aabb1.1.x < aabb2.0.x || aabb1.0.x > aabb2.1.x || 
      aabb1.1.y < aabb2.0.y || aabb1.0.y > aabb2.1.y || 
      aabb1.1.z < aabb2.0.z || aabb1.0.z > aabb2.1.z)
}

fn point_inside_aabb(point: &Point, min: &Point, max: &Point) -> bool {
    point.x >= min.x && point.x <= max.x &&
    point.y >= min.y && point.y <= max.y &&
    point.z >= min.z && point.z <= max.z
}

pub struct CubeCollider {
    pub side_length: f32,
}

impl CubeCollider {
    pub fn new(side_length: f32) -> Box<dyn Collider> {
        Box::new(Self { side_length })
    }
}

impl Collider for CubeCollider {
    fn points(&self) -> Vec<Point> {
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
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

impl RectangularPrismCollider {
    pub fn new(width: f32, height: f32, depth: f32) -> Box<dyn Collider> {
        Box::new(Self { width, height, depth })
    }
}

impl Collider for RectangularPrismCollider {
    fn points(&self) -> Vec<Point> {
        let (half_width, half_height, half_depth) = (self.width / 2.0, self.height / 2.0, self.depth / 2.0);
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

#[derive(Clone, Debug)]
pub struct PointCollider {
    pub point: Point,
}

impl PointCollider {
    pub fn new(point: Point) -> Box<dyn Collider> {
        Box::new(Self { point })
    }
}

impl Collider for PointCollider {
    fn points(&self) -> Vec<Point> {
        vec![self.point.clone()]
    }

    fn intersects(&self, line_segment: &(Point, Point), current_pos: &Point) -> bool {
        let point = Point {
            x: self.point.x + current_pos.x,
            y: self.point.y + current_pos.y,
            z: self.point.z + current_pos.z,
        };

        point_on_line_segment(&line_segment.0, &line_segment.1, &point)
    }
}

fn point_on_line_segment(p1: &Point, p2: &Point, p: &Point) -> bool {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);
    let min_z = p1.z.min(p2.z);
    let max_z = p1.z.max(p2.z);

    (p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y && p.z >= min_z && p.z <= max_z)
        && orientation(p1, p2, p) == 0
}

#[derive(Clone, Debug)]
pub struct OctagonCollider {
    pub size: f32,
}

impl OctagonCollider {
    pub fn new(size: f32) -> Box<dyn Collider> {
        Box::new(Self { size })
    }
}

impl Collider for OctagonCollider {
    fn points(&self) -> Vec<Point> {
        let angle = 2.0 * std::f32::consts::PI / 8.0; // 45 degrees in radians
        (0..8)
            .map(|i| {
                let theta = i as f32 * angle;
                Point {
                    x: self.size * theta.cos(),
                    y: self.size * theta.sin(),
                    z: 0.0,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    pub struct SimpleCollider {
        points: Vec<Point>,
    }

    impl SimpleCollider {
        pub fn new(points: Vec<Point>) -> Box<dyn Collider> {
            Box::new(Self { points })
        }
    }

    impl Collider for SimpleCollider {
        fn points(&self) -> Vec<Point> {
            self.points.clone()
        }
    }

    #[test]
    fn test_line_segment_intersects_true_case() {
        let p1 = Point { x: 1.0, y: 1.0, z: 0.0 };
        let p2 = Point { x: 4.0, y: 4.0, z: 0.0 };
        let p3 = Point { x: 1.0, y: 4.0, z: 0.0 };
        let p4 = Point { x: 4.0, y: 1.0, z: 0.0 };
        assert!(line_segment_intersects(&p1, &p2, &p3, &p4));
    }

    #[test]
    fn test_line_segment_intersects_false_case() {
        let p1 = Point { x: 1.0, y: 1.0, z: 0.0 };
        let p2 = Point { x: 2.0, y: 2.0, z: 0.0 };
        let p3 = Point { x: 3.0, y: 3.0, z: 0.0 };
        let p4 = Point { x: 4.0, y: 4.0, z: 0.0 };
        assert!(!line_segment_intersects(&p1, &p2, &p3, &p4));
    }

    #[test]
    fn test_line_segment_intersects_collinear_case() {
        let p1 = Point { x: 1.0, y: 1.0, z: 0.0 };
        let p2 = Point { x: 5.0, y: 5.0, z: 0.0 };
        let p3 = Point { x: 3.0, y: 3.0, z: 0.0 };
        let p4 = Point { x: 7.0, y: 7.0, z: 0.0 };
        assert!(line_segment_intersects(&p1, &p2, &p3, &p4));
    }

    #[test]
    fn test_collider_intersects_vertical_line() {
        let square_points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 4.0, z: 0.0 },
            Point { x: 0.0, y: 4.0, z: 0.0 },
        ];
        let square_collider = SimpleCollider::new(square_points);

        let line_segment = (
            Point { x: 2.0, y: -1.0, z: 0.0 },
            Point { x: 2.0, y: 5.0, z: 0.0 },
        );
        assert!(square_collider.intersects(&line_segment, &Point { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_collider_intersects_outside_line() {
        let square_points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 4.0, z: 0.0 },
            Point { x: 0.0, y: 4.0, z: 0.0 },
        ];
        let square_collider = SimpleCollider::new(square_points);

        let line_segment = (
            Point { x: -1.0, y: -1.0, z: 0.0 },
            Point { x: -2.0, y: -2.0, z: 0.0 },
        );
        assert!(!square_collider.intersects(&line_segment, &Point { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_collider_intersects_horizontal_line() {
        let square_points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 4.0, z: 0.0 },
            Point { x: 0.0, y: 4.0, z: 0.0 },
        ];
        let square_collider = SimpleCollider::new(square_points);

        let line_segment = (
            Point { x: 0.0, y: 2.0, z: 0.0 },
            Point { x: 4.0, y: 2.0, z: 0.0 },
        );
        assert!(square_collider.intersects(&line_segment, &Point { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_collider_intersects_diagonal_line() {
        let square_points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 4.0, z: 0.0 },
            Point { x: 0.0, y: 4.0, z: 0.0 },
        ];
        let square_collider = SimpleCollider::new(square_points);

        let line_segment = (
            Point { x: 1.0, y: 1.0, z: 0.0 },
            Point { x: 3.0, y: 3.0, z: 0.0 },
        );
        assert!(square_collider.intersects(&line_segment, &Point { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_collider_colliding_with_overlapping() {
        let square_points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 4.0, z: 0.0 },
            Point { x: 0.0, y: 4.0, z: 0.0 },
        ];
        let square_collider = Arc::new(Mutex::new(SimpleCollider::new(square_points)));

        let other_points = vec![
            Point { x: 1.0, y: 1.0, z: 0.0 },
            Point { x: 2.0, y: 1.0, z: 0.0 },
            Point { x: 2.0, y: 2.0, z: 0.0 },
            Point { x: 1.0, y: 2.0, z: 0.0 },
        ];
        let other_collider = Arc::new(Mutex::new(SimpleCollider::new(other_points)));

        let current_pos = Point { x: 0.0, y: 0.0, z: 0.0 };
        let other_pos = Point { x: 1.0, y: 1.0, z: 0.0 };
        assert!(square_collider.lock().unwrap().colliding_with(&current_pos, other_collider.clone(), &other_pos));
    }

    #[test]
    fn test_collider_colliding_with_touching_edges() {
        let square_points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 4.0, z: 0.0 },
            Point { x: 0.0, y: 4.0, z: 0.0 },
        ];
        let square_collider = Arc::new(Mutex::new(SimpleCollider::new(square_points)));

        let other_points = vec![
            Point { x: 1.0, y: 1.0, z: 0.0 },
            Point { x: 2.0, y: 1.0, z: 0.0 },
            Point { x: 2.0, y: 2.0, z: 0.0 },
            Point { x: 1.0, y: 2.0, z: 0.0 },
        ];
        let other_collider = Arc::new(Mutex::new(SimpleCollider::new(other_points)));

        let current_pos = Point { x: 5.0, y: 5.0, z: 0.0 };
        let other_pos = Point { x: 6.0, y: 6.0, z: 0.0 };
        assert!(square_collider.lock().unwrap().colliding_with(&current_pos, other_collider.clone(), &other_pos));
    }

    #[test]
    fn test_collider_not_colliding_with() {
        let square_points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 4.0, z: 0.0 },
            Point { x: 0.0, y: 4.0, z: 0.0 },
        ];
        let square_collider = Arc::new(Mutex::new(SimpleCollider::new(square_points)));

        let other_points = vec![
            Point { x: 1.0, y: 1.0, z: 0.0 },
            Point { x: 2.0, y: 1.0, z: 0.0 },
            Point { x: 2.0, y: 2.0, z: 0.0 },
            Point { x: 1.0, y: 2.0, z: 0.0 },
        ];
        let other_collider = Arc::new(Mutex::new(SimpleCollider::new(other_points)));

        let current_pos = Point { x: 15.0, y: 5.0, z: 0.0 };
        let other_pos = Point { x: 6.0, y: 6.0, z: 0.0 };
        assert!(!square_collider.lock().unwrap().colliding_with(&current_pos, other_collider.clone(), &other_pos));
    }

    #[test]
    fn test_collider_colliding_point_inside() {
        let square_points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 4.0, z: 0.0 },
            Point { x: 0.0, y: 4.0, z: 0.0 },
        ];
        let square_collider = SimpleCollider::new(square_points);

        let current_pos = Point { x: 0.0, y: 0.0, z: 0.0 };
        let point = Point { x: 2.0, y: 2.0, z: 0.0 };
        assert!(square_collider.colliding_point(&current_pos, &point));
    }

    #[test]
    fn test_collider_colliding_point_outside() {
        let square_points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 4.0, z: 0.0 },
            Point { x: 0.0, y: 4.0, z: 0.0 },
        ];
        let square_collider = SimpleCollider::new(square_points);

        let current_pos = Point { x: 0.0, y: 0.0, z: 0.0 };
        let point = Point { x: 5.0, y: 5.0, z: 0.0 };
        assert!(!square_collider.colliding_point(&current_pos, &point));
    }

    #[test]
    fn test_point_collider_intersects() {
        let point = Point { x: 1.0, y: 1.0, z: 1.0 };
        let collider = PointCollider::new(point.clone());
        let line_segment = (Point { x: 0.0, y: 0.0, z: 0.0 }, Point { x: 2.0, y: 2.0, z: 2.0 });
        assert!(collider.intersects(&line_segment, &Point { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_point_collider_does_not_intersect() {
        let point = Point { x: 5.0, y: 5.0, z: 5.0 };
        let collider = PointCollider::new(point.clone());
        let line_segment = (Point { x: 0.0, y: 0.0, z: 0.0 }, Point { x: 2.0, y: 2.0, z: 2.0 });
        assert!(!collider.intersects(&line_segment, &Point { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_point_collider_colliding_with() {
        let point1 = Point { x: 1.0, y: 1.0, z: 1.0 };
        let point2 = Point { x: 1.0, y: 1.0, z: 1.0 };
        let collider1 = Arc::new(Mutex::new(PointCollider::new(point1.clone())));
        let collider2 = Arc::new(Mutex::new(PointCollider::new(point2.clone())));
        assert!(collider1.lock().unwrap().colliding_with(&point1, collider2, &point2));
    }

    #[test]
    fn test_point_collider_not_colliding_with() {
        let point1 = Point { x: 1.0, y: 1.0, z: 1.0 };
        let point2 = Point { x: 2.0, y: 2.0, z: 2.0 };
        let collider1 = Arc::new(Mutex::new(PointCollider::new(point1.clone())));
        let collider2 = Arc::new(Mutex::new(PointCollider::new(point2.clone())));
        assert!(!collider1.lock().unwrap().colliding_with(&point1, collider2, &point2));
    }

    #[test]
    fn test_point_collider_colliding_point() {
        let point = Point { x: 1.0, y: 1.0, z: 1.0 };
        let collider = PointCollider::new(point.clone());
        assert!(collider.colliding_point(&Point { x: 0.0, y: 0.0, z: 0.0 }, &point));
    }

    #[test]
    fn test_point_collider_not_colliding_point() {
        let point1 = Point { x: 1.0, y: 1.0, z: 1.0 };
        let point2 = Point { x: 3.0, y: 2.0, z: 2.0 };
        let collider = PointCollider::new(point1.clone());
        assert!(!collider.colliding_point(&point1, &point2));
    }

    #[test]
    fn test_octagon_collider_points() {
        let size = 2.0;
        let octagon_collider = OctagonCollider::new(size);
        let points = octagon_collider.points();
        assert_eq!(points.len(), 8);
    }

    #[test]
    fn test_octagon_collider_intersects() {
        let octagon_collider = OctagonCollider::new(2.0);
        let line_segment = (
            Point { x: -1.0, y: -1.0, z: 0.0 },
            Point { x: 1.0, y: 1.0, z: 0.0 },
        );
        assert!(octagon_collider.intersects(&line_segment, &Point { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_octagon_collider_does_not_intersect() {
        let octagon_collider = OctagonCollider::new(2.0);
        let line_segment = (
            Point { x: 3.0, y: 3.0, z: 0.0 },
            Point { x: 4.0, y: 4.0, z: 0.0 },
        );
        assert!(!octagon_collider.intersects(&line_segment, &Point { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_octagon_collider_colliding_with() {
        let octagon_collider = Arc::new(Mutex::new(OctagonCollider::new(2.0)));
        let other_collider = Arc::new(Mutex::new(OctagonCollider::new(2.0)));

        let current_pos = Point { x: 0.0, y: 0.0, z: 0.0 };
        let other_pos = Point { x: 1.0, y: 1.0, z: 0.0 };
        assert!(octagon_collider.lock().unwrap().colliding_with(&current_pos, other_collider.clone(), &other_pos));
    }

    #[test]
    fn test_octagon_collider_not_colliding_with() {
        let octagon_collider = Arc::new(Mutex::new(OctagonCollider::new(2.0)));
        let other_collider = Arc::new(Mutex::new(OctagonCollider::new(2.0)));

        let current_pos = Point { x: 0.0, y: 0.0, z: 0.0 };
        let other_pos = Point { x: 5.0, y: 5.0, z: 0.0 };
        assert!(!octagon_collider.lock().unwrap().colliding_with(&current_pos, other_collider.clone(), &other_pos));
    }

    #[test]
    fn test_octagon_collider_colliding_point() {
        let octagon_collider = OctagonCollider::new(2.0);
        let point = Point { x: 0.5, y: 0.5, z: 0.0 };
        assert!(octagon_collider.colliding_point(&Point { x: 0.0, y: 0.0, z: 0.0 }, &point));
    }

    #[test]
    fn test_octagon_collider_not_colliding_point() {
        let octagon_collider = OctagonCollider::new(2.0);
        let point = Point { x: 3.0, y: 3.0, z: 0.0 };
        assert!(!octagon_collider.colliding_point(&Point { x: 0.0, y: 0.0, z: 0.0 }, &point));
    }
}
