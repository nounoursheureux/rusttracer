use cgmath::Vector3;
use cgmath::prelude::*;

pub type Vec3f = Vector3<f32>;

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

