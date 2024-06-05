use crate::engine::collider::Point;
use crate::engine::graphics_backend::object::{BufferDesc, Object};
use crate::engine::graphics_backend::Vertex;
use std::f32::consts::PI;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Primitives {
    Cube(f32, [f32; 3]),
    Triangle(f32, [f32; 3]),
    Hexagon(f32, [f32; 3]),
    Pentagon(f32, [f32; 3]),
    Octagon(f32, [f32; 3]),
    Line(Point, f32, f32, f32, [f32; 3]),
    RaycastLine(Point, f32, f32, f32, [f32; 3]),
}

impl Default for Primitives {
    fn default() -> Self {
        Self::Cube(0.0, [0.0,0.0,0.0])
    }
}

#[derive(Clone)]
pub struct Cube {
    vertex: Vec<Vertex>,
    index: Vec<u16>,
}

#[derive(Clone)]
pub struct Triangle {
    vertex: Vec<Vertex>,
    index: Vec<u16>,
}

#[derive(Clone)]
pub struct Hexagon {
    vertex: Vec<Vertex>,
    index: Vec<u16>,
}

#[derive(Clone)]
pub struct Pentagon {
    vertex: Vec<Vertex>,
    index: Vec<u16>,
}

#[derive(Clone)]
pub struct Octagon {
    vertex: Vec<Vertex>,
    index: Vec<u16>,
}

#[derive(Clone)]
pub struct Line {
    vertex: Vec<Vertex>,
    index: Vec<u16>,
}

#[derive(Clone)]
pub struct RaycastLine {
    vertex: Vec<Vertex>,
    index: Vec<u16>,
}

impl Object for Cube {
    fn desc(&mut self) -> BufferDesc {
        BufferDesc {
            vertex: self.vertex.clone(),
            index: self.index.clone(),
        }
    }

    fn get_vertexes(&mut self) -> Vec<Vertex> {
        self.vertex.clone()
    }

    fn set_vertexes(&mut self, vertexes: Vec<Vertex>) {
        self.vertex = vertexes
    }
}

impl Object for Triangle {
    fn desc(&mut self) -> BufferDesc {
        BufferDesc {
            vertex: self.vertex.clone(),
            index: self.index.clone(),
        }
    }

    fn get_vertexes(&mut self) -> Vec<Vertex> {
        self.vertex.clone()
    }

    fn set_vertexes(&mut self, vertexes: Vec<Vertex>) {
        self.vertex = vertexes
    }
}

impl Object for Hexagon {
    fn desc(&mut self) -> BufferDesc {
        BufferDesc {
            vertex: self.vertex.clone(),
            index: self.index.clone(),
        }
    }

    fn get_vertexes(&mut self) -> Vec<Vertex> {
        self.vertex.clone()
    }

    fn set_vertexes(&mut self, vertexes: Vec<Vertex>) {
        self.vertex = vertexes
    }
}

impl Object for Pentagon {
    fn desc(&mut self) -> BufferDesc {
        BufferDesc {
            vertex: self.vertex.clone(),
            index: self.index.clone(),
        }
    }

    fn get_vertexes(&mut self) -> Vec<Vertex> {
        self.vertex.clone()
    }

    fn set_vertexes(&mut self, vertexes: Vec<Vertex>) {
        self.vertex = vertexes
    }
}

impl Object for Octagon {
    fn desc(&mut self) -> BufferDesc {
        BufferDesc {
            vertex: self.vertex.clone(),
            index: self.index.clone(),
        }
    }

    fn get_vertexes(&mut self) -> Vec<Vertex> {
        self.vertex.clone()
    }

    fn set_vertexes(&mut self, vertexes: Vec<Vertex>) {
        self.vertex = vertexes
    }
}

impl Cube {
    pub fn new(len: f32, color: [f32; 3]) -> Self {
        let half_len = len / 2.0;
        let vertex = vec![
            Vertex {
                position: [-half_len, -half_len, half_len],
                color,
            }, // Vertex 0, Front-bottom-left
            Vertex {
                position: [half_len, -half_len, half_len],
                color,
            }, // Vertex 1, Front-bottom-right
            Vertex {
                position: [half_len, half_len, half_len],
                color,
            }, // Vertex 2, Front-top-right
            Vertex {
                position: [-half_len, half_len, half_len],
                color,
            }, // Vertex 3, Front-top-left
            Vertex {
                position: [-half_len, -half_len, -half_len],
                color,
            }, // Vertex 4, Back-bottom-left
            Vertex {
                position: [half_len, -half_len, -half_len],
                color,
            }, // Vertex 5, Back-bottom-right
            Vertex {
                position: [half_len, half_len, -half_len],
                color,
            }, // Vertex 6, Back-top-right
            Vertex {
                position: [-half_len, half_len, -half_len],
                color,
            }, // Vertex 7, Back-top-left
        ];

        let index = vec![
            0, 1, 2, 2, 3, 0, // Front face
            1, 5, 6, 6, 2, 1, // Right face
            7, 6, 5, 5, 4, 7, // Back face
            4, 0, 3, 3, 7, 4, // Left face
            4, 5, 1, 1, 0, 4, // Bottom face
            3, 2, 6, 6, 7, 3, // Top face
        ];

        Cube { vertex, index }
    }
}

impl Triangle {
    pub fn new(size: f32, color: [f32; 3]) -> Self {
        let half_size = size / 2.0;
        let vertex = vec![
            Vertex {
                position: [0.0, half_size, 0.0],
                color,
            }, // Vertex 0, Top
            Vertex {
                position: [-half_size, -half_size, 0.0],
                color,
            }, // Vertex 1, Bottom-left
            Vertex {
                position: [half_size, -half_size, 0.0],
                color,
            }, // Vertex 2, Bottom-right
        ];

        let index = vec![
            0, 1, 2, // Triangle face
        ];

        Triangle { vertex, index }
    }
}

impl Hexagon {
    pub fn new(size: f32, color: [f32; 3]) -> Self {
        let angle = std::f32::consts::PI / 3.0;
        let vertex = (0..6).map(|i| {
            let theta = i as f32 * angle;
            Vertex {
                position: [size * theta.cos(), size * theta.sin(), 0.0],
                color,
            }
        }).collect::<Vec<_>>();

        let index = vec![
            0, 1, 2, // First triangle
            2, 3, 0, // Second triangle
            3, 4, 0, // Third triangle
            4, 5, 0, // Fourth triangle
            5, 1, 0, // Fifth triangle
        ];

        Hexagon { vertex, index }
    }
}

impl Pentagon {
    pub fn new(size: f32, color: [f32; 3]) -> Self {
        let angle = 2.0 * std::f32::consts::PI / 5.0;
        let vertex = (0..5).map(|i| {
            let theta = i as f32 * angle;
            Vertex {
                position: [size * theta.cos(), size * theta.sin(), 0.0],
                color,
            }
        }).collect::<Vec<_>>();

        let index = vec![
            0, 1, 2, // First triangle
            2, 3, 0, // Second triangle
            3, 4, 0, // Third triangle
        ];

        Pentagon { vertex, index }
    }
}

impl Octagon {
    pub fn new(size: f32, color: [f32; 3]) -> Self {
        let angle = 2.0 * std::f32::consts::PI / 8.0;
        let vertex = (0..8).map(|i| {
            let theta = i as f32 * angle;
            Vertex {
                position: [size * theta.cos(), size * theta.sin(), 0.0],
                color,
            }
        }).collect::<Vec<_>>();

        let index = vec![
            0, 1, 2, // First triangle
            2, 3, 0, // Second triangle
            3, 4, 0, // Third triangle
            4, 5, 0, // Fourth triangle
            5, 6, 0, // Fifth triangle
            6, 7, 0, // Sixth triangle
        ];

        Octagon { vertex, index }
    }
}

impl Object for Line {
    fn desc(&mut self) -> BufferDesc {
        BufferDesc {
            vertex: self.vertex.clone(),
            index: self.index.clone(),
        }
    }

    fn get_vertexes(&mut self) -> Vec<Vertex> {
        self.vertex.clone()
    }

    fn set_vertexes(&mut self, vertexes: Vec<Vertex>) {
        self.vertex = vertexes
    }
}

impl Line {
    pub fn new(start_point: Point, angle: f32, length: f32, width: f32, color: [f32; 3]) -> Self {
        let half_width = width / 2.0;
        let end_x = start_point.x + length * angle.cos();
        let end_y = start_point.y + length * angle.sin();

        let vertex = vec![
            Vertex {
                position: [start_point.x - half_width * angle.sin(), start_point.y + half_width * angle.cos(), start_point.z],
                color,
            }, // Vertex 0, Start-left
            Vertex {
                position: [start_point.x + half_width * angle.sin(), start_point.y - half_width * angle.cos(), start_point.z],
                color,
            }, // Vertex 1, Start-right
            Vertex {
                position: [end_x + half_width * angle.sin(), end_y - half_width * angle.cos(), start_point.z],
                color,
            }, // Vertex 2, End-right
            Vertex {
                position: [end_x - half_width * angle.sin(), end_y + half_width * angle.cos(), start_point.z],
                color,
            }, // Vertex 3, End-left
        ];

        let index = vec![
            0, 1, 2, 2, 3, 0, // Line quad
        ];

        Line { vertex, index }
    }
}

impl RaycastLine {
    pub fn new(start_point: Point, angle: f32, length: f32, thickness: f32, color: [f32; 3]) -> Self {
        let angle_rad = angle * (PI / 180.0); // Convert to radians
        let half_thickness = thickness / 2.0;
        let end_x = start_point.x + length * angle_rad.cos();
        let end_y = start_point.y + length * angle_rad.sin();

        let vertex = vec![
            Vertex {
                position: [start_point.x - half_thickness * angle_rad.sin(), start_point.y + half_thickness * angle_rad.cos(), start_point.z],
                color,
            }, // Vertex 0, Start-left
            Vertex {
                position: [start_point.x + half_thickness * angle_rad.sin(), start_point.y - half_thickness * angle_rad.cos(), start_point.z],
                color,
            }, // Vertex 1, Start-right
            Vertex {
                position: [end_x + half_thickness * angle_rad.sin(), end_y - half_thickness * angle_rad.cos(), start_point.z],
                color,
            }, // Vertex 2, End-right
            Vertex {
                position: [end_x - half_thickness * angle_rad.sin(), end_y + half_thickness * angle_rad.cos(), start_point.z],
                color,
            }, // Vertex 3, End-left
        ];

        let index = vec![
            0, 1, 2, 2, 3, 0, // Line quad
        ];

        RaycastLine { vertex, index }
    }
}

impl Object for RaycastLine {
    fn desc(&mut self) -> BufferDesc {
        BufferDesc {
            vertex: self.vertex.clone(),
            index: self.index.clone(),
        }
    }

    fn get_vertexes(&mut self) -> Vec<Vertex> {
        self.vertex.clone()
    }

    fn set_vertexes(&mut self, vertexes: Vec<Vertex>) {
        self.vertex = vertexes
    }
}
