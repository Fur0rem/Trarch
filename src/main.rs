use std::rc::Rc;

use scene::Scene;

use crate::{
    math::{Quat, Vec3},
    shape::{Object, Shape},
};

mod camera;
mod math;
mod scene;
mod shape;

fn main() {
    const WIDTH: u32 = 1000;
    const HEIGHT: u32 = 500;
    let mut scene = Scene::empty();
    scene.camera.set_aspect_ratio(WIDTH, HEIGHT);
    let mut object1 = Object::new(
        Vec3::new(-3.0, 0.0, -4.0),
        Quat::identity(),
        Vec3::new(1.0, 1.0, 1.0),
        Shape::Sphere,
    );
    object1.fragment_shader = Rc::new(|point| Vec3::new(1.0, 0.0, 0.0));
    scene.add_object(object1);

    let mut object2 = Object::new(
        Vec3::new(3.0, 0.0, -4.0),
        Quat::rot_y(0.5),
        Vec3::new(1.0, 2.0, 1.0),
        Shape::Cube,
    );
    object2.fragment_shader = Rc::new(|point| Vec3::new(0.0, 1.0, 0.0));
    scene.add_object(object2);

    let mut object3 = Object::new(
        Vec3::new(0.0, 0.0, -4.0),
        Quat::rot_x(0.5),
        Vec3::new(1.0, 1.0, 1.0),
        Shape::Mandelbulb {
            iterations: 8,
            power: 8.0,
        },
    );
    object3.fragment_shader = Rc::new(|point| Vec3::new(0.0, 0.0, 1.0));
    object3.set_inflate(0.02);
    scene.add_object(object3);

    scene.camera.rotate(0.0, 0.0);
    let render = scene.render(WIDTH, HEIGHT);
    render.to_png(WIDTH, HEIGHT, "renders/output3");

    println!("{scene:?}");
}
