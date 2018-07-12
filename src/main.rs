extern crate cgmath;
extern crate tobj;

mod image;
mod shape;
mod types;
mod util;
mod mesh;

use cgmath::vec3;
use cgmath::prelude::*;
use std::path::Path;
use image::{Image, Color};
use shape::{Shape, Sphere, Triangle};
use types::{Ray, Vec3f};
use util::*;
use mesh::Mesh;

fn main() {
	let path = Path::new(r"out.png");
	let sphere = Sphere {
		center: Vec3f{ x: 0.0, y: 0.0, z: 0.0},
		radius: 0.3,
	};
	let suzanne = Mesh::load(Path::new(r"suzanne.obj")).unwrap();
	println!("{}", suzanne.vertices.len());
	let triangle = Triangle::new(vec3(-1.0, 1.0, 0.0), vec3(-1.0, -1.0, 0.0), vec3(0.0, 0.0, 0.0));
	let width: u32 = 500;
	let height: u32 = 500;
	let mut pixels: Vec<Color> = vec![Color { r: 0.0, g: 0.0, b: 0.0}; (width * height) as usize];
	let camera_pos = Vec3f { x: 0.0, y: 0.0, z: 1.0};
	let light_pos = Vec3f { x: -0.5, y: 0.0, z: 1.0};
	for pixel_y in 0..height {
		let h2 = height / 2;
		for pixel_x in 0..width {
			let w2 = width / 2;
			let screen_x = (pixel_x as i32 - w2 as i32) as f32 / w2 as f32;
			let screen_y = (h2 as i32 - pixel_y as i32 ) as f32 / h2 as f32;
			let screen_pos = Vec3f { x: screen_x, y: screen_y, z: 0.0 };
			let camera_ray = Ray::new(camera_pos, screen_pos - camera_pos);
			// if sphere.get_closest_hit(camera_ray).is_some() {
			// 	let t = sphere.get_closest_hit(camera_ray).unwrap();
			// 	let inter = camera_ray.origin + t * camera_ray.direction;
			// 	let N = sphere.normal(inter);
			// 	let L = -camera_ray.direction;
			// 	let intensity = L.dot(N);
			// 	pixels[(pixel_y * width + pixel_x) as usize] = Color { r: 1.0 * intensity, g: 0.0, b: 0.0 };
			// } else if triangle.get_closest_hit(camera_ray).is_some() {
			// 	pixels[(pixel_y * width + pixel_x) as usize] = Color { r: 0.0, g: 1.0, b: 0.0 };
			// }

			if let Some(inter) = suzanne.intersects(camera_ray) {
				let L = (light_pos - inter.position).normalize();
				let intensity = L.dot(inter.normal);
				pixels[(pixel_y * width + pixel_x) as usize] = Color::new(1.0 * intensity, 0.0, 0.0);
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
