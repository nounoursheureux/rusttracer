extern crate cgmath;

mod image;

use cgmath::Vector3;
use cgmath::prelude::*;
use std::path::Path;
use image::{Image, Color};

type Vec3f = Vector3<f32>;

struct Ray {
	origin: Vec3f,
	direction: Vec3f
}

#[derive(PartialEq)]
enum EqSolution {
	Zero,
	One(f32),
	Two(f32, f32)
}

fn solve_quadratic_equation(a: f32, b: f32, c: f32) -> EqSolution {
	let delta = b * b - 4.0 * a * c;
	if delta < 0.0 {
		EqSolution::Zero
	} else if delta == 0.0 {
		EqSolution::One(-b/(2.0*a))
	} else {
		EqSolution::Two((-b-delta.sqrt())/(2.0*a),(-b+delta.sqrt())/(2.0*a))
	}
}

#[test]
fn test_solve_quadratic_equation()
{
	assert!(solve_quadratic_equation(1.0, -2.0, -3.0) == EqSolution::Two(-1.0, 3.0));
	assert!(solve_quadratic_equation(3.0, -6.0, 3.0) == EqSolution::One(1.0));
	assert!(solve_quadratic_equation(1.0, 1.0, 1.0) == EqSolution::Zero);
}

trait Shape {
	fn normal(&self, point: Vec3f) -> Vec3f;
	fn get_closest_hit(&self, ray: Ray) -> Option<f32>;
}

struct Sphere {
	center: Vec3f,
	radius: f32
}

impl Shape for Sphere {
	fn normal(&self, point: Vec3f) -> Vec3f {
		(point - self.center).normalize()
	}

	fn get_closest_hit(&self, ray: Ray) -> Option<f32> {
		let a = cgmath::dot(ray.direction, ray.direction);
		let c_o = self.center - ray.origin;
		let b = 2.0 * cgmath::dot(ray.direction, c_o);
		let c = cgmath::dot(c_o, c_o) - self.radius * self.radius;
		match solve_quadratic_equation(a, b, c) {
			EqSolution::Zero => None,
			EqSolution::One(t1) => Some(t1),
			EqSolution::Two(t1, t2) => Some(f32::min(t1, t2))
		}
	}
}

fn main() {
	let path = Path::new(r"out.png");
	let mut pixels: Vec<Color> = Vec::new();
	let sphere = Sphere {
		center: Vec3f{ x: 0.0, y: 0.0, z: 0.0},
		radius: 0.3,
	};
	let width: i32 = 100;
	let height: i32 = 100;
	let camera_pos = Vec3f { x: 0.0, y: 0.0, z: 1.0};
	for pixel_x in 0..width {
		let w2 = width / 2;
		for pixel_y in 0..height {
			let h2 = height / 2;
			let screen_x = (pixel_x - w2) as f32 / w2 as f32;
			let screen_y = (pixel_y - h2) as f32 / h2 as f32;
			let screen_pos = Vec3f { x: screen_x, y: screen_y, z: 0.0 };
			let camera_ray = Ray {
				origin: camera_pos,
				direction: screen_pos - camera_pos,
			};
			if sphere.get_closest_hit(camera_ray).is_some() {
				pixels.push(Color { r: 1.0, g: 0.0, b: 0.0 });
			} else {
				pixels.push(Color { r: 0.0, g: 0.0, b: 1.0 });
			}
		}
	}
	let img = Image {
		width: width as u32,
		height: height as u32,
		data: pixels
	};
	img.write(path).unwrap();
}
