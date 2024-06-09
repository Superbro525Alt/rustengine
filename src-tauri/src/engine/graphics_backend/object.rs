use crate::engine::graphics_backend::vertex::Vertex;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct BufferDesc {
    pub vertex: Vec<Vertex>,
    pub index: Vec<u16>,
}

impl BufferDesc {
    pub fn get_raw(&mut self) -> (Vec<Vertex>, Vec<u16>) {
        (self.vertex.clone(), self.index.clone())
    }
}

pub trait Object: Send + Sync {
    fn desc(&mut self) -> BufferDesc;
    fn move_vertexes(&mut self, pos: [f32; 3]) -> BufferDesc {
        let mut vs = self.get_vertexes();
        for v in vs.iter_mut() {
            v.position[0] += pos[0];
            v.position[1] += pos[1];
            v.position[2] += pos[2];
        }

        self.set_vertexes(vs);

        self.desc()
    }

    fn rotate_vertexes(&mut self, angle_deg: f32, axis: char, camera_pos: [f32; 3]) -> BufferDesc {
        let angle_rad = angle_deg * PI / 180.0;
        let sin_angle = angle_rad.sin();
        let cos_angle = angle_rad.cos();
        let mut vertexes = self.get_vertexes();

        // Calculate the center of the object
        let mut center = [0.0, 0.0, 0.0];
        let vertex_count = vertexes.len() as f32;
        for vertex in &vertexes {
            center[0] += vertex.position[0];
            center[1] += vertex.position[1];
            center[2] += vertex.position[2];
        }
        center[0] /= vertex_count;
        center[1] /= vertex_count;
        center[2] /= vertex_count;

        // Translate vertices to origin
        for vertex in vertexes.iter_mut() {
            vertex.position[0] -= center[0];
            vertex.position[1] -= center[1];
            vertex.position[2] -= center[2];
        }

        // Rotate vertices around the origin
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

        // Translate vertices back to their original position
        for vertex in vertexes.iter_mut() {
            vertex.position[0] += center[0];
            vertex.position[1] += center[1];
            vertex.position[2] += center[2];
        }

        self.set_vertexes(vertexes);
        self.desc()
    }

    fn rotate_vertexes_arr(&mut self, arr: [f32; 3], camera_pos: [f32; 3]) {
        self.rotate_vertexes(arr[0], 'x', camera_pos);
        self.rotate_vertexes(arr[1], 'y', camera_pos);
        self.rotate_vertexes(arr[2], 'z', camera_pos);
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
