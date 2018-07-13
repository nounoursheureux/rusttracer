use cgmath::{Vector3, Matrix4, Point3, Point2};
use cgmath::prelude::*;

pub type Vec3f = Vector3<f32>;
pub type Matrix4f = Matrix4<f32>;
pub type Point2f = Point2<f32>;
pub type Point3f = Point3<f32>;

#[derive(Copy, Clone)]
pub struct Ray {
	pub origin: Vec3f,
	pub direction: Vec3f
}

impl Ray {
	pub fn new(origin: Vec3f, direction: Vec3f) -> Ray {
		Ray {
			origin: origin,
			direction: direction.normalize()
		}
	}

	pub fn at(&self, t: f32) -> Vec3f {
		self.origin + t * self.direction
	}
}

