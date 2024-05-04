use crate::math::{Quat, Vec3};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::rc::Rc;

pub struct Object {
    pub shape: Shape,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub inflate: f64,
    pub fragment_shader: Rc<dyn Fn(Vec3) -> Vec3>,
    pub vertex_shader: Rc<dyn Fn(Vec3) -> Vec3>,
}

impl Clone for Object {
    fn clone(&self) -> Object {
        Object {
            shape: self.shape,
            position: self.position,
            rotation: self.rotation,
            scale: self.scale,
            inflate: self.inflate,
            fragment_shader: self.fragment_shader.clone(),
            vertex_shader: self.vertex_shader.clone(),
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Object {{ position: {:?}, rotation: {:?}, scale: {:?}, shape: {:?} }}",
            self.position, self.rotation, self.scale, self.shape
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Sphere,
    Cube,
    Mandelbulb { iterations: u32, power: f64 },
}

impl Object {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3, shape: Shape) -> Object {
        Object {
            position,
            rotation,
            scale,
            shape,
            inflate: 0.0,
            fragment_shader: Rc::new(|point| Vec3::new(1.0, 0.0, 1.0)),
            vertex_shader: Rc::new(|point| point),
        }
    }

    pub fn distance(&self, point: Vec3) -> f64 {
        // translate
        let point = (point - self.position);
        // rotate
        let point = self.rotation.conjugate().rotate(point);
        // scale
        let point = point / self.scale;

        let dist = match self.shape {
            Shape::Sphere => point.length() - 1.0,
            Shape::Cube => {
                let d = point.abs() - 1.0;
                d.max(Vec3::new(0.0, 0.0, 0.0)).length()
                    + d.min(Vec3::new(0.0, 0.0, 0.0)).max_element()
            }
            Shape::Mandelbulb { iterations, power } => {
                let mut z = point;
                let mut dr = 1.0;
                let mut r = 0.0;

                for _ in 0..iterations {
                    r = z.length();
                    if r > 2.0 {
                        break;
                    }

                    // Convert to polar coordinates
                    let mut theta = (z.z / r).acos();
                    let mut phi = z.y.atan2(z.x);
                    dr = r.powf(power - 1.0) * power * dr + 1.0;

                    // Scale and rotate the point
                    let zr = r.powf(power);
                    theta *= power;
                    phi *= power;

                    // Convert back to cartesian coordinates
                    z = Vec3 {
                        x: zr * theta.sin() * phi.cos(),
                        y: zr * theta.sin() * phi.sin(),
                        z: zr * theta.cos(),
                    } + point;

                    z = z * self.scale;
                }

                0.5 * r.ln() * r / dr
            }
        };

        return dist - self.inflate;
    }

    pub fn set_fragment_shader(&mut self, fragment_shader: Rc<dyn Fn(Vec3) -> Vec3>) {
        self.fragment_shader = fragment_shader;
    }

    pub fn set_vertex_shader(&mut self, vertex_shader: Rc<dyn Fn(Vec3) -> Vec3>) {
        self.vertex_shader = vertex_shader;
    }

    pub fn set_inflate(&mut self, inflate: f64) {
        self.inflate = inflate;
    }
}
