use glm::{mat4, Matrix4};
use super::utils::{deg_to_rad, rotate, matrix4_to_array};
use super::types::{Vector};

use super::input_state::InputState;

const F_NEAR: f32 = 0.1;
const F_FAR: f32 = 100.0;
const F_FOV: f32 = 90.0;

#[derive(Debug)]
pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub position: Vector,
    pub rotation: Vector,
    pub speed: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, near: f32, far: f32, fov: f32) -> Camera {
        Camera {
            aspect_ratio: aspect_ratio, 
            near: near,
            far: far,
            fov: fov,
            position: Vector::new(0.3,-1.4,1.0),
            rotation: Vector::new(0.0, 0.0, 0.0),
            speed: 5.0,
        }
    }

    pub fn update(&mut self, input: &InputState, delta_time: f32) {
        if input.forward.is_down {
            self.position.z += self.speed * delta_time;
        }
        if input.back.is_down {
            self.position.z -= self.speed * delta_time;
        }
        if input.left.is_down {
            self.position.x -= self.speed * delta_time;
        }
        if input.right.is_down {
            self.position.x += self.speed * delta_time;
        }
        if input.look_up.is_down {
            self.rotation.x += self.speed * delta_time;
        }
        if input.look_down.is_down {
            self.rotation.x -= self.speed * delta_time;
        }
        if input.look_left.is_down {
            self.rotation.y -= self.speed * delta_time;
        }
        if input.look_right.is_down {
            self.rotation.y += self.speed * delta_time;
        }
        if input.up.is_down {
            self.position.y -= self.speed * delta_time;
        }
        if input.down.is_down {
            self.position.y += self.speed * delta_time;
        }
    }

    pub fn reset(&mut self) {
        self.rotation = Vector::new(0.0, 0.0, 0.0);
        self.position = Vector::new(0.0, 0.0, 0.0);
    }

    pub fn move_to(&mut self, x: f32, y: f32, z: f32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }

    pub fn projection(&self) -> [f32; 16] {
        let proj = self.create_projection_matrix();
        matrix4_to_array(proj)
    }

    fn create_projection_matrix(&self) -> Matrix4<f32> {
        let fov_rad = deg_to_rad(self.fov * 0.5); 
        let fov = 1.0 / fov_rad.tan();
        let x = fov / self.aspect_ratio;
        let y = fov;
        let z = (self.far + self.near) / (self.near - self.far);
        let w = 2.0 * self.far * self.near / (self.near - self.far);

        let projection = mat4(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, -1.0,
            0.0, 0.0, w, 0.0,
        );
        let view = mat4(
            1.0,    0.0,    0.0,    0.0,
            0.0,    1.0,    0.0,    0.0,
            0.0,    0.0,    1.0,    0.0,
            self.position.x, self.position.y, self.position.z, 1.0,
        );
        // let view = mat4(
        //     1.0,    0.0,    0.0,    self.position.x,
        //     0.0,    1.0,    0.0,    self.position.y,
        //     0.0,    0.0,    1.0,    self.position.z,
        //     0.0,    0.0,    0.0,    1.0,
        // );
        // let view = mat4(
        //     1.0,    0.0,    0.0,    self.position.x,
        //     0.0,    1.0,    0.0,    self.position.y,
        //     0.0,    0.0,    1.0,    self.position.z,
        //     self.position.x, self.position.y, self.position.z, 1.0,
        // );
        // let opengl_fix = mat4(
        //     1.0, 0.0, 0.0, 0.0,
        //     0.0, 1.0, 0.0, 0.0,
        //     0.0, 0.0, 0.5, 0.0,
        //     0.0, 0.0, 0.5, 1.0,
        // );

        projection * rotate(&self.rotation) * view
    }
}