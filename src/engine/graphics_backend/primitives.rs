use crate::engine::graphics_backend::object::{BufferDesc, Object};
use crate::engine::graphics_backend::Vertex;

pub struct Cube {
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
