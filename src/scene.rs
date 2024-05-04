//! A scene is a collection of shapes and a camera.

use crate::camera::Camera;
use crate::math::{Quat, Vec3};
use crate::shape::{Object, Shape};

use image::RgbaImage;

/*pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
}

pub fn smooth_min(a: f64, b: f64, k: f64) -> f64 {
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

    pub fn distance(&self, point: Vec3) -> f64 {
        self.objects
            .iter()
            .map(|object| object.distance(point))
            .fold(f64::INFINITY, f64::min)
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}

pub struct RenderImage {
    pub pixels: Vec<Vec<(f64, f64, f64, f64)>>,
}

pub struct Render {
    pub colours: Vec<Vec<(f64, f64, f64, f64)>>,
    pub ambient: Vec<Vec<(f64, f64, f64, f64)>>,
}

impl Scene {
    pub fn render(&self, width: u32, height: u32) -> Render {
        let mut colours = vec![vec![(0.0, 0.0, 0.0, 1.0); width as usize]; height as usize];
        let mut ambient = vec![vec![(0.0, 0.0, 0.0, 1.0); width as usize]; height as usize];

        for py in 0..height {
            for px in 0..width {
                let x = px as f64 / width as f64;
                let y = py as f64 / height as f64;

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

                let occ = 1.0 - (distance / 1000.0).min(1.0) - (iterations as f64 / 25.0).min(1.0);
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
    pub fn to_png(&self, width: u32, height: u32, dir_name: &str) {
        std::fs::create_dir_all(dir_name).unwrap();

        let mut image = RgbaImage::new(width, height);
        let mut colours = RgbaImage::new(width, height);
        let mut ambient = RgbaImage::new(width, height);
        let mut depthi = RgbaImage::new(width, height);
        let mut m = RgbaImage::new(width, height);

        for x in 0..width {
            for y in 0..height {
                let (or, og, ob, _) = self.colour[y as usize][x as usize];
                let occl = self.steps[y as usize][x as usize];
                let depth = self.depth[y as usize][x as usize];
                let mind = self.min_distance[y as usize][x as usize];

                let r = (or * 255.0).round() as u8;
                let g = (og * 255.0).round() as u8;
                let b = (ob * 255.0).round() as u8;
                let occ = (occl * 255.0).round() as u8;
                let depthu: u8 = 255 - (depth / 10.0 * 255.0).round() as u8;
                let mind_powed = (1.0 - mind).powf(25.0);
                //println!("{}", mind);
                let mind = 255 - (mind * 255.0).min(255.0).round() as u8;
                //println!("{}", mind);

                m.put_pixel(x, y, image::Rgba([mind, mind, mind, 255]));
                colours.put_pixel(x, y, image::Rgba([r, g, b, 255]));
                ambient.put_pixel(x, y, image::Rgba([occ, occ, occ, 255]));
                depthi.put_pixel(x, y, image::Rgba([depthu, depthu, depthu, 255]));

                let occl = occl.powf(3.0);
                let r = ((r as f64 / 255.0) * occl).min(1.0);
                let g = ((g as f64 / 255.0) * occl).min(1.0);
                let b = ((b as f64 / 255.0) * occl).min(1.0);

                let depth = 1.0 - (depth / 30.0).min(1.0);
                let r = r * depth;
                let g = g * depth;
                let b = b * depth;
                let mut r = (r * 255.0).round() as u8;
                let mut g = (g * 255.0).round() as u8;
                let mut b = (b * 255.0).round() as u8;
                if or == 0.0 && og == 0.0 && ob == 0.0 {
                    r = (mind_powed * 255.0).round() as u8;
                    g = (mind_powed * 255.0).round() as u8;
                    b = (mind_powed * 255.0).round() as u8;
                }
                image.put_pixel(x, y, image::Rgba([r, g, b, 255]));
            }
        }

        image.save(format!("{}/final.png", dir_name)).unwrap();
        colours.save(format!("{}/colours.png", dir_name)).unwrap();
        ambient.save(format!("{}/steps.png", dir_name)).unwrap();
        depthi.save(format!("{}/depth.png", dir_name)).unwrap();
        m.save(format!("{}/min_distance.png", dir_name)).unwrap();
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
    SmoothUnion(f64),
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
    pub pixels: Vec<Vec<(f64, f64, f64, f64)>>,
}

#[derive(Clone, Debug)]

pub struct Render {
    pub colour: Vec<Vec<(f64, f64, f64, f64)>>,
    pub steps: Vec<Vec<f64>>,
    pub depth: Vec<Vec<f64>>,
    pub min_distance: Vec<Vec<f64>>,
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
                (45.0_f64).to_radians(),
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

    pub fn distance(&self, point: Vec3) -> f64 {
        self.distance_recursive(&self.scene, point)
    }

    fn distance_recursive(&self, node: &TreeNode, point: Vec3) -> f64 {
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

    pub fn distance_and_colour(&self, point: Vec3) -> (f64, Vec3) {
        self.distance_and_colour_recursive(&self.scene, point)
    }

    fn distance_and_colour_recursive(&self, node: &TreeNode, point: Vec3) -> (f64, Vec3) {
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
                    /*Operation::SmoothUnion(k) => {
                        let h = (k - (left_dist - right_dist).abs()).max(0.0) / k;
                        let new_min = left_dist.min(right_dist) - h * h * k * (1.0 / 5.0);
                        //mix between the two colours
                        let how_close_to_left =
                            (left_dist - new_min) / (left_dist - right_dist).abs();
                        //println!("{}", how_close_to_left);
                        let how_close_to_right = 1.0 - how_close_to_left;
                        let col = left_col * how_close_to_left + right_col * how_close_to_right;

                        (new_min, col)
                    }*/
                    Operation::SmoothUnion(k) => {
                        let h = (k - (left_dist - right_dist).abs()).max(0.0) / k;
                        let new_min = left_dist.min(right_dist) - h * h * k * (1.0 / 5.0);
                        //mix between the two colours
                        let how_close_to_left =
                            (left_dist - new_min) / (left_dist - right_dist).abs();
                        //println!("{}", how_close_to_left);
                        let how_close_to_right = 1.0 - how_close_to_left;

                        // mix between the two colours with k as the factor
                        let mean_col = (left_col + right_col) / 2.0;
                        let col = left_col * how_close_to_right + right_col * how_close_to_left;
                        let col = mean_col * (h) + col * (1.0 - h);

                        (new_min, col)
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

    pub fn set_first_object(&mut self, object: Object) {
        self.scene = TreeNode::Leaf(object);
    }

    pub fn render(&self, width: u32, height: u32) -> Render {
        let mut colours = vec![vec![(0.0, 0.0, 0.0, 1.0); width as usize]; height as usize];
        let mut occl = vec![vec![0.0; width as usize]; height as usize];
        let mut depth = vec![vec![0.0; width as usize]; height as usize];
        let mut min_distances = vec![vec![100000.0; width as usize]; height as usize];

        for py in 0..height {
            for px in 0..width {
                let x = px as f64 / width as f64;
                let y = py as f64 / height as f64;

                let ray = self.camera.ray(x, y);
                let mut t = 0.0;
                let mut distance = 100000.0;
                let mut colour = Vec3::new(0.0, 0.0, 0.0);
                let mut total_distance = 0.0;
                let mut min_distance = 100000.0f64;
                let mut iterations = 0;
                for _ in 0..500 {
                    let point = ray.point(t);
                    let (d, col) = self.distance_and_colour(point);
                    colour = col;
                    distance = d;
                    t += distance;
                    iterations += 1;
                    total_distance += distance;
                    min_distance = min_distance.min(distance);
                    if distance < 0.001 || distance > 1000.0 {
                        break;
                    }
                }

                let occ = 1.0 - (iterations as f64 / 500.0).min(1.0);
                let colour = if distance < 0.001 {
                    colour
                } else {
                    Vec3::new(0.0, 0.0, 0.0)
                };
                let colour = (colour.x, colour.y, colour.z, 1.0);
                min_distances[py as usize][px as usize] = min_distance;
                colours[py as usize][px as usize] = colour;
                occl[py as usize][px as usize] = occ;
                depth[py as usize][px as usize] = total_distance;
            }
        }

        Render {
            colour: colours,
            steps: occl,
            depth,
            min_distance: min_distances,
        }
    }
}
