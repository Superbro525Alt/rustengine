pub enum Colors {
    RED,
    GREEN,
    BLUE,
    CUSTOM([f32; 3]),
}

impl Colors {
    pub fn raw(&self) -> [f32; 3] {
        match *self {
            Colors::RED => [1.0, 0.0, 0.0],
            Colors::GREEN => [0.0, 1.0, 0.0],
            Colors::BLUE => [0.0, 0.0, 1.0],
            Colors::CUSTOM(color) => color,
        }
    }
}
