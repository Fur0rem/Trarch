//! A scene is a collection of shapes and a camera.

use crate::camera::Camera;
use crate::math::{Quat, Vec3};
use crate::shape::{Object, Shape};

use image::RgbaImage;

/*pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
}

pub fn smooth_min(a: f32, b: f32, k: f32) -> f32 {
    if k < 1e-5 {
        return a.min(b);
    }
    let h = (k - (a - b).abs()).max(0.0) / k;
    let new_min = a.min(b) - h * h * k * (1.0 / 5.0);
    new_min
}

impl Scene {
    /// Create a new scene with a camera and a list of objects.
    pub fn new(camera: Camera, objects: Vec<Object>) -> Scene {
        Scene { camera, objects }
    }

    pub fn distance(&self, point: Vec3) -> f32 {
        self.objects
            .iter()
            .map(|object| object.distance(point))
            .fold(f32::INFINITY, f32::min)
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}

pub struct RenderImage {
    pub pixels: Vec<Vec<(f32, f32, f32, f32)>>,
}

pub struct Render {
    pub colours: Vec<Vec<(f32, f32, f32, f32)>>,
    pub ambient: Vec<Vec<(f32, f32, f32, f32)>>,
}

impl Scene {
    pub fn render(&self, width: u32, height: u32) -> Render {
        let mut colours = vec![vec![(0.0, 0.0, 0.0, 1.0); width as usize]; height as usize];
        let mut ambient = vec![vec![(0.0, 0.0, 0.0, 1.0); width as usize]; height as usize];

        for py in 0..height {
            for px in 0..width {
                let x = px as f32 / width as f32;
                let y = py as f32 / height as f32;

                let ray = self.camera.ray(x, y);
                let mut t = 0.0;
                let mut distance = 100000.0;
                let mut closest_shape = &self.objects[0];

                let mut iterations = 0;
                for _ in 0..500 {
                    distance = 500000.0;
                    let point = ray.point(t);
                    for object in &self.objects {
                        let d = object.distance(point);
                        /*if d < distance {
                            distance = d;
                            closest_shape = object;
                        }*/
                        distance = smooth_min(distance, d, object.blending);
                    }
                    t += distance;
                    iterations += 1;
                    if distance < 0.001 || distance > 1000.0 {
                        break;
                    }
                }

                let occ = 1.0 - (distance / 1000.0).min(1.0) - (iterations as f32 / 25.0).min(1.0);
                let color = if distance < 0.001 {
                    closest_shape.color
                } else {
                    Vec3::new(0.0, 0.0, 0.0)
                };
                let color = (color.x, color.y, color.z, 1.0);

                colours[py as usize][px as usize] = color;
                ambient[py as usize][px as usize] = (occ, occ, occ, 1.0);
            }
        }

        Render { colours, ambient }
    }
}*/

impl Render {
    pub fn to_ppm(&self, width: u32, height: u32) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", width, height);

        for x in 0..width {
            for y in 0..height {
                let (r, g, b, _) = self.colour[y as usize][x as usize];
                let o = self.occlusion[y as usize][x as usize];
                let r = (r * o * 255.0).round() as u8;
                let g = (g * o * 255.0).round() as u8;
                let b = (b * o * 255.0).round() as u8;
                ppm.push_str(&format!("{} {} {}\n", r, g, b));
            }
        }

        ppm
    }

    pub fn to_png(&self, width: u32, height: u32, dir_name: &str) {
        std::fs::create_dir_all(dir_name).unwrap();

        let mut image = RgbaImage::new(width, height);
        let mut colours = RgbaImage::new(width, height);
        let mut ambient = RgbaImage::new(width, height);
        let mut depthi = RgbaImage::new(width, height);

        for x in 0..width {
            for y in 0..height {
                let (r, g, b, _) = self.colour[y as usize][x as usize];
                let occ = self.occlusion[y as usize][x as usize];
                let depth = self.depth[y as usize][x as usize];

                let r = (r * 255.0).round() as u8;
                let g = (g * 255.0).round() as u8;
                let b = (b * 255.0).round() as u8;
                let occ = (occ * 255.0).round() as u8;
                let depth = 255 - (depth / 5.0 * 255.0).round() as u8;

                colours.put_pixel(x, y, image::Rgba([r, g, b, 255]));
                ambient.put_pixel(x, y, image::Rgba([occ, occ, occ, 255]));
                depthi.put_pixel(x, y, image::Rgba([depth, depth, depth, 255]));

                let r = (r as f32 / 255.0) * (occ as f32 / 155.0 * depth as f32 / 255.0).min(1.0);
                let g = (g as f32 / 255.0) * (occ as f32 / 155.0 * depth as f32 / 255.0).min(1.0);
                let b = (b as f32 / 255.0) * (occ as f32 / 155.0 * depth as f32 / 255.0).min(1.0);
                let r = (r * 255.0).round() as u8;
                let g = (g * 255.0).round() as u8;
                let b = (b * 255.0).round() as u8;
                image.put_pixel(x, y, image::Rgba([r, g, b, 255]));
            }
        }

        image.save(format!("{}/final.png", dir_name)).unwrap();
        colours.save(format!("{}/colours.png", dir_name)).unwrap();
        ambient.save(format!("{}/ambient.png", dir_name)).unwrap();
        depthi.save(format!("{}/depth.png", dir_name)).unwrap();
    }
}

/*impl Scene {
    pub fn default() -> Scene {
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            16.0 / 9.0,
        );

        let objects = vec![
            Object::new(
                Vec3::new(0.0, 0.0, -5.0),
                Quat::identity(),
                Vec3::new(1.0, 1.0, 1.0),
                Shape::Sphere,
            ),
            Object::new(
                Vec3::new(0.0, 0.0, -5.0),
                Quat::identity(),
                Vec3::new(1.0, 1.0, 1.0),
                Shape::Sphere,
            ),
        ];

        Scene::new(camera, objects)
    }

    pub fn empty() -> Scene {
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            16.0 / 9.0,
        );

        Scene::new(camera, vec![])
    }
}*/

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Union,
    SmoothUnion(f32),
    Intersection,
}

#[derive(Clone, Debug)]
pub enum TreeNode {
    Leaf(Object),
    Node(ObjectTree),
}

#[derive(Clone, Debug)]
pub struct ObjectTree {
    pub operation: Operation,
    pub left: Box<TreeNode>,
    pub right: Box<TreeNode>,
}

#[derive(Clone, Debug)]

pub struct Scene {
    pub camera: Camera,
    pub scene: TreeNode,
}

#[derive(Clone, Debug)]

pub struct RenderImage {
    pub pixels: Vec<Vec<(f32, f32, f32, f32)>>,
}

#[derive(Clone, Debug)]

pub struct Render {
    pub colour: Vec<Vec<(f32, f32, f32, f32)>>,
    pub occlusion: Vec<Vec<f32>>,
    pub depth: Vec<Vec<f32>>,
}

impl Scene {
    pub fn new(camera: Camera, scene: TreeNode) -> Scene {
        Scene { camera, scene }
    }

    pub fn empty() -> Scene {
        Scene {
            camera: Camera::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 0.0),
                90.0,
                16.0 / 9.0,
            ),
            scene: TreeNode::Leaf(Object::new(
                Vec3::new(0.0, 0.0, -5.0),
                Quat::identity(),
                Vec3::new(1.0, 1.0, 1.0),
                Shape::Sphere,
            )),
        }
    }

    pub fn distance(&self, point: Vec3) -> f32 {
        self.distance_recursive(&self.scene, point)
    }

    fn distance_recursive(&self, node: &TreeNode, point: Vec3) -> f32 {
        match node {
            TreeNode::Leaf(object) => object.distance(point),
            TreeNode::Node(tree) => {
                let left = self.distance_recursive(&*tree.left, point);
                let right = self.distance_recursive(&*tree.right, point);

                match tree.operation {
                    Operation::Union => left.min(right),
                    Operation::SmoothUnion(k) => {
                        let h = (k - (left - right).abs()).max(0.0) / k;
                        let new_min = left.min(right) - h * h * k * (1.0 / 5.0);
                        new_min
                    }
                    Operation::Intersection => left.max(right),
                }
            }
        }
    }

    pub fn distance_and_colour(&self, point: Vec3) -> (f32, Vec3) {
        self.distance_and_colour_recursive(&self.scene, point)
    }

    fn distance_and_colour_recursive(&self, node: &TreeNode, point: Vec3) -> (f32, Vec3) {
        match node {
            TreeNode::Leaf(object) => {
                let dist = object.distance((object.vertex_shader)(point));
                let col = (object.fragment_shader)(point);
                (dist, col)
            }
            TreeNode::Node(tree) => {
                let (left_dist, left_col) = self.distance_and_colour_recursive(&*tree.left, point);
                let (right_dist, right_col) =
                    self.distance_and_colour_recursive(&*tree.right, point);

                match tree.operation {
                    Operation::Union => {
                        if left_dist < right_dist {
                            (left_dist, left_col)
                        } else {
                            (right_dist, right_col)
                        }
                    }
                    Operation::SmoothUnion(k) => {
                        let h = (k - (left_dist - right_dist).abs()).max(0.0) / k;
                        let new_min = left_dist.min(right_dist) - h * h * k * (1.0 / 5.0);
                        if left_dist < right_dist {
                            (new_min, left_col)
                        } else {
                            (new_min, right_col)
                        }
                    }
                    Operation::Intersection => {
                        if left_dist > right_dist {
                            (left_dist, left_col)
                        } else {
                            (right_dist, right_col)
                        }
                    }
                }
            }
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.scene = TreeNode::Node(ObjectTree {
            operation: Operation::Union,
            left: Box::new(self.scene.clone()),
            right: Box::new(TreeNode::Leaf(object)),
        });
    }

    pub fn render(&self, width: u32, height: u32) -> Render {
        let mut colours = vec![vec![(0.0, 0.0, 0.0, 1.0); width as usize]; height as usize];
        let mut occl = vec![vec![0.0; width as usize]; height as usize];
        let mut depth = vec![vec![0.0; width as usize]; height as usize];

        for py in 0..height {
            for px in 0..width {
                let x = px as f32 / width as f32;
                let y = py as f32 / height as f32;

                let ray = self.camera.ray(x, y);
                let mut t = 0.0;
                let mut distance = 100000.0;
                let mut colour = Vec3::new(0.0, 0.0, 0.0);
                let mut total_distance = 0.0;
                let mut iterations = 0;
                for _ in 0..500 {
                    distance = 500000.0;
                    let point = ray.point(t);
                    let (d, col) = self.distance_and_colour(point);
                    colour = col;
                    distance = d;
                    t += distance;
                    iterations += 1;
                    total_distance += distance;
                    if distance < 0.001 || distance > 1000.0 {
                        break;
                    }
                }

                let occ = 1.0 - (iterations as f32 / 32.0).min(1.0);
                let colour = if distance < 0.001 {
                    colour
                } else {
                    Vec3::new(0.0, 0.0, 0.0)
                };
                let colour = (colour.x, colour.y, colour.z, 1.0);

                colours[py as usize][px as usize] = colour;
                occl[py as usize][px as usize] = occ;
                depth[py as usize][px as usize] = total_distance;
            }
        }

        Render {
            colour: colours,
            occlusion: occl,
            depth,
        }
    }
}
