use std::rc::Rc;

use scene::Scene;

use crate::{
    math::{Quat, Vec3},
    scene::{ObjectTree, TreeNode},
    shape::{Object, Shape},
};

mod camera;
mod math;
mod scene;
mod shape;

fn main() {
    const WIDTH: u32 = 2000;
    const HEIGHT: u32 = 1000;
    let mut scene = Scene::empty();
    scene.camera.set_aspect_ratio(WIDTH, HEIGHT);
    scene.camera.position = Vec3::new(0.0, 0.0, 1.0);
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
    object2.set_inflate(0.1);
    object2.fragment_shader = Rc::new(|point| Vec3::new(0.0, 1.0, 0.0));
    scene.add_object(object2);

    let mut object3 = Object::new(
        Vec3::new(0.0, 0.0, -4.0),
        Quat::rot_x(0.5),
        Vec3::new(1.0, 1.0, 1.0),
        Shape::Mandelbulb {
            iterations: 20,
            power: 8.0,
        },
    );
    object3.fragment_shader =
        Rc::new(|point| Vec3::new(0.4 * (5.0 * point.x).sin().min(1.0).max(0.0), 0.0, 1.0));
    object3.set_inflate(0.00);
    scene.add_object(object3);

    scene.camera.rotate(0.0, 0.0);
    let render = scene.render(WIDTH, HEIGHT);
    render.to_png(WIDTH, HEIGHT, "renders/output1");
    println!("{scene:?}");

    let mut scene = Scene::empty();
    let mut object1 = Object::new(
        Vec3::new(-1.0, 0.0, -4.0),
        Quat::identity(),
        Vec3::new(1.0, 1.0, 1.0),
        Shape::Sphere,
    );
    object1.fragment_shader = Rc::new(|point| Vec3::new(1.0, 0.0, 0.0));

    let mut object2 = Object::new(
        Vec3::new(1.0, 0.0, -4.0),
        Quat::rot_y(0.5),
        Vec3::new(1.0, 2.0, 1.0),
        Shape::Cube,
    );
    object2.set_inflate(0.1);
    object2.fragment_shader = Rc::new(|point| Vec3::new(0.0, 1.0, 0.0));

    scene.scene = TreeNode::Node(ObjectTree {
        operation: scene::Operation::SmoothUnion(0.5),
        left: Box::new(TreeNode::Leaf(object1)),
        right: Box::new(TreeNode::Leaf(object2)),
    });

    scene.camera.set_aspect_ratio(WIDTH, HEIGHT);
    scene.camera.position = Vec3::new(0.0, 0.0, 1.0);
    scene.camera.rotate(0.0, 0.0);
    let render = scene.render(WIDTH, HEIGHT);
    render.to_png(WIDTH, HEIGHT, "renders/output2");

    println!("{scene:?}");

    let mut scene = Scene::empty();
    let mut object1 = Object::new(
        Vec3::new(-1.0, 0.0, -4.0),
        Quat::identity(),
        Vec3::new(1.0, 1.0, 1.0),
        Shape::Sphere,
    );
    object1.fragment_shader = Rc::new(|point| Vec3::new(1.0, 0.0, 0.0));
    let mut object2 = Object::new(
        Vec3::new(1.0, 0.0, -4.0),
        Quat::rot_y(0.5),
        Vec3::new(1.0, 1.0, 1.0),
        Shape::Sphere,
    );
    object2.fragment_shader = Rc::new(|point| Vec3::new(0.0, 0.0, 1.0));
    scene.scene = TreeNode::Node(ObjectTree {
        operation: scene::Operation::SmoothUnion(2.0),
        left: Box::new(TreeNode::Leaf(object1)),
        right: Box::new(TreeNode::Leaf(object2)),
    });

    scene.camera.set_aspect_ratio(WIDTH, HEIGHT);
    scene.camera.position = Vec3::new(0.0, 0.0, 1.0);
    scene.camera.rotate(0.0, 0.0);
    let render = scene.render(WIDTH, HEIGHT);
    render.to_png(WIDTH, HEIGHT, "renders/output3");
}
