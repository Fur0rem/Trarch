//! Camera for a ray-marching renderer.

use crate::math::{Quat, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn point(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(position: Vec3, direction: Vec3, up: Vec3, fov: f32, aspect_ratio: f32) -> Camera {
        let right = direction.cross(up).normalize();
        let up = right.cross(direction).normalize();
        Camera {
            position,
            direction,
            up,
            right,
            fov,
            aspect_ratio,
        }
    }

    pub fn set_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }

    pub fn ray(&self, x: f32, y: f32) -> Ray {
        let x = 0.5 * (2.0 * x - 1.0) * self.aspect_ratio * self.fov.tan();
        let y = 0.5 * (1.0 - 2.0 * y) * self.fov.tan();
        Ray::new(self.position, self.direction + self.right * x + self.up * y)
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        let yaw = Quat::from_axis_angle(self.up, yaw);
        let pitch = Quat::from_axis_angle(self.right, pitch);
        self.direction = (yaw * pitch).rotate(self.direction);
        self.right = self.direction.cross(self.up).normalize();
        self.up = self.right.cross(self.direction).normalize();
    }

    pub fn move_forward(&mut self, amount: f32) {
        self.position += self.direction * amount;
    }

    pub fn move_right(&mut self, amount: f32) {
        self.position += self.right * amount;
    }
}
