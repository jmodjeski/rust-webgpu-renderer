// use glm::{vec2, vec3, Vector2, Vector3};
use zerocopy::{AsBytes, FromBytes};

#[repr(C)]
#[derive(Clone, Copy, AsBytes, FromBytes)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub const VERTEX_SIZE: usize = std::mem::size_of::<Vertex>();

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> Vertex {
        Vertex {
            position: [x, y, z],
            color: [r, g, b]
        }
    }

    #[allow(dead_code)]
    pub fn s_new(position: [f32; 3], color: [f32; 3]) -> Vertex {
        Vertex {
            position: position,
            color: color,
        }
    }
}

#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }
}

