use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct XLimits {
    pub x: f32,
}

impl XLimits {
    pub fn new(limit: f32) -> Self {
        Self { x: limit }
    }

    pub fn in_limit(&self, x: f32) -> bool {
        -self.x <= x && x <= self.x
    }

    pub fn clip(&self, x: f32) -> f32 {
        x.min(self.x).max(-self.x)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct YLimits {
    pub y: f32,
}

impl YLimits {
    pub fn new(limit: f32) -> Self {
        Self { y: limit }
    }

    pub fn in_limit(&self, y: f32) -> bool {
        -self.y <= y && y <= self.y
    }

    pub fn clip(&self, y: f32) -> f32 {
        y.min(self.y).max(-self.y)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ZLimits {
    pub z: f32,
}

impl ZLimits {
    pub fn new(limit: f32) -> Self {
        Self { z: limit }
    }

    pub fn in_limit(&self, z: f32) -> bool {
        -self.z <= z && z <= self.z
    }

    pub fn clip(&self, z: f32) -> f32 {
        z.min(self.z).max(-self.z)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Limits2D {
    pub x: XLimits,
    pub y: YLimits,
}

impl Limits2D {
    pub fn new(x_limit: f32, y_limit: f32) -> Self {
        Self {
            x: XLimits::new(x_limit),
            y: YLimits::new(y_limit),
        }
    }

    pub fn in_limits(&self, x: f32, y: f32) -> bool {
        self.x.in_limit(x) && self.y.in_limit(y)
    }

    pub fn in_x_limit(&self, x: f32) -> bool {
        self.x.in_limit(x)
    }

    pub fn in_y_limit(&self, y: f32) -> bool {
        self.y.in_limit(y)
    }

    pub fn clip(&self, x: f32, y: f32) -> [f32; 2] {
        [self.x.clip(x), self.y.clip(y)]
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Limits3D {
    pub x: XLimits,
    pub y: YLimits,
    pub z: ZLimits,
}

impl Limits3D {
    pub fn new(x_limit: f32, y_limit: f32, z_limit: f32) -> Self {
        Self {
            x: XLimits::new(x_limit),
            y: YLimits::new(y_limit),
            z: ZLimits::new(z_limit),
        }
    }

    pub fn in_limits(&self, x: f32, y: f32, z: f32) -> bool {
        self.x.in_limit(x) && self.y.in_limit(y) && self.z.in_limit(z)
    }

    pub fn in_x_limit(&self, x: f32) -> bool {
        self.x.in_limit(x)
    }

    pub fn in_y_limit(&self, y: f32) -> bool {
        self.y.in_limit(y)
    }

    pub fn in_z_limit(&self, z: f32) -> bool {
        self.z.in_limit(z)
    }

    pub fn clip(&self, x: f32, y: f32, z: f32) -> [f32; 3] {
        [self.x.clip(x), self.y.clip(y), self.z.clip(z)]
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Bounds2D {
    pub limits: Limits2D,
}

impl Bounds2D {
    pub fn new(x_limit: f32, y_limit: f32) -> Self {
        Self {
            limits: Limits2D::new(x_limit, y_limit),
        }
    }

    pub fn in_bounds(&self, x: f32, y: f32) -> bool {
        self.limits.in_limits(x, y)
    }

    pub fn in_x_bound(&self, x: f32) -> bool {
        self.limits.in_x_limit(x)
    }

    pub fn in_y_bound(&self, y: f32) -> bool {
        self.limits.in_y_limit(y)
    }

    pub fn clip(&self, x: f32, y: f32) -> [f32; 2] {
        self.limits.clip(x, y)
    }

    pub fn x(&mut self) -> f32 {
        self.limits.x.x
    }

    pub fn y(&mut self) -> f32 {
        self.limits.y.y
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Bounds3D {
    pub limits: Limits3D,
}

impl Bounds3D {
    pub fn new(x_limit: f32, y_limit: f32, z_limit: f32) -> Self {
        Self {
            limits: Limits3D::new(x_limit, y_limit, z_limit),
        }
    }

    pub fn in_bounds(&self, x: f32, y: f32, z: f32) -> bool {
        self.limits.in_limits(x, y, z)
    }

    pub fn in_x_bound(&self, x: f32) -> bool {
        self.limits.in_x_limit(x)
    }

    pub fn in_y_bound(&self, y: f32) -> bool {
        self.limits.in_y_limit(y)
    }

    pub fn in_z_bound(&self, z: f32) -> bool {
        self.limits.in_z_limit(z)
    }

    pub fn clip(&self, x: f32, y: f32, z: f32) -> [f32; 3] {
        self.limits.clip(x, y, z)
    }
}
