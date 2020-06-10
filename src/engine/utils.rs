use std::convert::TryInto;
use std::f32::consts::PI;
use glm::{mat4, Matrix4};
use super::types::{Vector};

pub fn deg_to_rad(deg: f32) -> f32 {
    (deg * PI) / 180.0
}

pub fn rotation(v: &Vector) -> Matrix4<f32> {
    xrotation(v.x) * yrotation(v.y) * zrotation(v.z)
}

pub fn trasform(pos: &Vector, rot: &Vector) -> Matrix4<f32> {
    let x_sin = rot.x.sin();
    let x_cos = rot.x.cos();
    let y_sin = rot.y.sin();
    let y_cos = rot.y.cos();
    let z_sin = rot.z.sin();
    let z_cos = rot.z.cos();

    mat4(
        z_cos * y_cos, (z_cos * y_sin * x_sin) - (z_sin * x_cos), (z_cos * y_sin * x_cos) + (z_sin * x_sin), 0.0,
        z_sin * y_cos, (z_sin * y_sin * x_sin) + (z_cos * x_cos), (z_sin * y_sin * x_cos) - (z_cos * x_sin), 0.0,
        -y_sin, y_cos * x_sin, y_cos * x_cos,                                                                0.0,
        pos.x, pos.y, pos.z, 1.0
    )
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