use std::convert::TryInto;
use std::f32::consts::PI;
use glm::{mat4, Matrix4};

pub fn deg_to_rad(deg: f32) -> f32 {
    (deg * PI) / 180.0
}

pub fn rotation(x: f32, y: f32, z: f32) -> Matrix4<f32> {
    xrotation(x) * yrotation(y) * zrotation(z)
}

pub fn xrotation(rads: f32) -> Matrix4<f32> {
    let sincos = (rads.sin(), rads.cos());
    mat4(
        1.0,    0.0,       0.0,       0.0,
        0.0,    sincos.1,  -sincos.0, 0.0,
        0.0,    sincos.0,  sincos.1,  0.0,
        0.0,    0.0,       0.0,       1.0,
    )
}

pub fn yrotation(rads: f32) -> Matrix4<f32> {
    let sincos = (rads.sin(), rads.cos());
    mat4(
        sincos.1,    0.0,   sincos.0,  0.0,
        0.0,         1.0,   0.0,       0.0,
        -sincos.0,   0.0,   sincos.1,  0.0,
        0.0,         0.0,   0.0,       1.0,
    )
}

pub fn zrotation(rads: f32) -> Matrix4<f32> {
    let sincos = (rads.sin(), rads.cos());
    mat4(
        sincos.1,   -sincos.0,  0.0,   0.0,
        sincos.0,    sincos.1,  0.0,   0.0,
        0.0,         0.0,       1.0,   0.0,
        0.0,         0.0,       0.0,   1.0,
    )
}

pub fn matrix4_to_array(mat: Matrix4<f32>) -> [f32; 16] {
    let vecs = mat.as_array();
    let mut vals: Vec<[f32; 4]> = Vec::new();
    for v in vecs.iter() {
        vals.push(v.as_array().clone());
    }
    vals.concat()[..].try_into().expect("slice with incorrect length")
}