use glm::{mat4, Matrix4};
use super::utils::{deg_to_rad, rotation, matrix4_to_array};

const F_NEAR: f32 = 0.1;
const F_FAR: f32 = 100.0;
const F_FOV: f32 = 90.0;

#[derive(Debug)]
pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub speed: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, near: f32, far: f32, fov: f32) -> Camera {
        Camera {
            aspect_ratio: aspect_ratio, 
            near: near,
            far: far,
            fov: fov,
            x: 0.3,
            y: -1.4,
            z: 1.0,
            speed: 1.0,
        }
    }

    pub fn moveTo(&mut self, x: f32, y: f32, z: f32) {

    }

    pub fn rotate(&mut self) {
        let xrot = deg_to_rad(83.0);
        let yrot = deg_to_rad(21.0);
    }

    pub fn projection(&self) -> [f32; 16] {
        let proj = self.create_projection_matrix();
        matrix4_to_array(proj)
    }

    fn create_projection_matrix(&self) -> Matrix4<f32> {
        let fov_rad = deg_to_rad(self.fov * 0.5); 
        let fov = 1.0 / fov_rad.tan();
        let x = self.aspect_ratio * fov;
        let y = fov;
        let z = self.far / (self.far - self.near);
        let w = (-self.far * self.near) / (self.far - self.near);

        let projection = mat4(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 1.0,
            0.0, 0.0, w, 0.0,
        );
        let view = mat4(
            1.0,    0.0,    0.0,    0.0,
            0.0,    1.0,    0.0,    0.0,
            0.0,    0.0,    1.0,    0.0,
            self.x, self.y, self.z, 1.0,
        );
        // let opengl_fix = mat4(
        //     1.0, 0.0, 0.0, 0.0,
        //     0.0, 1.0, 0.0, 0.0,
        //     0.0, 0.0, 0.5, 0.0,
        //     0.0, 0.0, 0.5, 1.0,
        // );

        projection * view
    }
}