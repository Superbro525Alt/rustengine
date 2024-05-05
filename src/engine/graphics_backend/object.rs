use crate::engine::graphics_backend::vertex::Vertex;
use std::f32::consts::PI;

pub struct BufferDesc {
    pub vertex: Vec<Vertex>,
    pub index: Vec<u16>,
}

impl BufferDesc {
    pub fn get_raw(&mut self) -> (Vec<Vertex>, Vec<u16>) {
        (self.vertex.clone(), self.index.clone())
    }
}
pub trait Object {
    fn desc(&mut self) -> BufferDesc;
    fn move_vertexes(&mut self, pos: [f32; 3]) -> BufferDesc {
        for v in self.get_vertexes().iter_mut() {
            v.position[0] += pos[0];
            v.position[1] += pos[1];
            v.position[2] += pos[2];
        }

        self.desc()
    }
    fn rotate_vertexes(&mut self, angle_deg: f32, axis: char) -> BufferDesc {
        let angle_rad = angle_deg * PI / 180.0;
        let sin_angle = angle_rad.sin();
        let cos_angle = angle_rad.cos();
        let mut vertexes = self.get_vertexes();

        for vertex in vertexes.iter_mut() {
            let (x, y, z) = (vertex.position[0], vertex.position[1], vertex.position[2]);
            vertex.position = match axis {
                'x' => [
                    x,
                    y * cos_angle - z * sin_angle,
                    y * sin_angle + z * cos_angle,
                ],
                'y' => [
                    x * cos_angle + z * sin_angle,
                    y,
                    z * cos_angle - x * sin_angle,
                ],
                'z' => [
                    x * cos_angle - y * sin_angle,
                    x * sin_angle + y * cos_angle,
                    z,
                ],
                _ => [x, y, z],
            };
        }
        self.set_vertexes(vertexes);
        self.desc()
    }

    fn scale_vertexes(&mut self, scale: f32) -> BufferDesc {
        let mut vertexes = self.get_vertexes();
        for v in vertexes.iter_mut() {
            v.position[0] *= scale;
            v.position[1] *= scale;
            v.position[2] *= scale;
        }
        self.set_vertexes(vertexes);
        self.desc()
    }
    fn get_vertexes(&mut self) -> Vec<Vertex>;
    fn set_vertexes(&mut self, vertexes: Vec<Vertex>);
    fn desc_raw(&mut self) -> (Vec<Vertex>, Vec<u16>) {
        self.desc().get_raw()
    }
}
