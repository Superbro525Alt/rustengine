---
title: Graphics
description: Graphics
---

## Objects

The `obj` field is a `struct` that implements the following trait:

```rust
pub trait Object: Send + Sync {
    fn desc(&mut self) -> BufferDesc;
    fn get_vertexes(&mut self) -> Vec<Vertex>;
    fn set_vertexes(&mut self, vertexes: Vec<Vertex>);
}
```

:::note 
There are other methods that can be overridden but are already implemented by default 
```rust
pub trait Object: Send + Sync {
    fn move_vertexes(&mut self, pos: [f32; 3]) -> BufferDesc;
    fn rotate_vertexes(&mut self, angle_deg: f32, axis: char, camera_pos: [f32; 3]) -> BufferDesc;
    fn rotate_vertexes_arr(&mut self, arr: [f32; 3], camera_pos: [f32; 3]);
    fn scale_vertexes(&mut self, scale: f32) -> BufferDesc;
    fn desc_raw(&mut self) -> (Vec<Vertex>, Vec<u16>);
}
```
:::

A BufferDesc is an object that describes both a vertex buffer and an index buffer:
- The vertex buffer stores a list of vertices that define a shape.
- The index buffer contains the indices that determine how the vertices connect to form triangles.

## Square Example

### Step 1: Define the Square Struct

The Square struct will hold the vertex and index buffers.

```rust
use crate::engine::graphics_backend::vertex::Vertex;
use crate::engine::graphics_backend::{BufferDesc, Object};

pub struct Square {
    vertexes: Vec<Vertex>,
    indexes: Vec<u16>,
}

impl Square {
    pub fn new() -> Self {
        let vertexes = vec![
            Vertex { position: [-0.5,  0.5, 0.0] }, // Top-left
            Vertex { position: [-0.5, -0.5, 0.0] }, // Bottom-left
            Vertex { position: [ 0.5, -0.5, 0.0] }, // Bottom-right
            Vertex { position: [ 0.5,  0.5, 0.0] }, // Top-right
        ];

        let indexes = vec![
            0, 1, 2, // First triangle
            0, 2, 3, // Second triangle
        ];

        Self { vertexes, indexes }
    }
}
```
### Step 2: Implement the Object Trait

To integrate with the rendering system, we must implement Object for Square.

```rust
impl Object for Square {
    fn desc(&mut self) -> BufferDesc {
        BufferDesc {
            vertex: self.vertexes.clone(),
            index: self.indexes.clone(),
        }
    }

    fn get_vertexes(&mut self) -> Vec<Vertex> {
        self.vertexes.clone()
    }

    fn set_vertexes(&mut self, vertexes: Vec<Vertex>) {
        self.vertexes = vertexes;
    }
}
```

