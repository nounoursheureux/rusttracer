use cgmath;
use cgmath::prelude::*;
use cgmath::vec3;
use util::*;
use types::*;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vec3f,
    pub normal: Vec3f
}

#[derive(Copy, Clone)]
pub struct AABB {
	pub min: Vec3f,
	pub max: Vec3f,
}

pub trait Shape {
	fn normal(&self, point: Vec3f) -> Vec3f;
	fn get_closest_hit(&self, ray: Ray) -> Option<f32>;
}

pub struct Sphere {
	pub center: Vec3f,
	pub radius: f32
}

impl Shape for Sphere {
	fn normal(&self, point: Vec3f) -> Vec3f {
		(point - self.center).normalize()
	}

	// fn get_closest_hit(&self, ray: Ray) -> Option<f32> {
	// 	let a = cgmath::dot(ray.direction, ray.direction);
	// 	let c_o = self.center - ray.origin;
	// 	let b = 2.0 * cgmath::dot(ray.direction, c_o);
	// 	let c = cgmath::dot(c_o, c_o) - self.radius * self.radius;
	// 	match solve_quadratic_equation(a, b, c) {
	// 		EqSolution::Zero => None,
	// 		EqSolution::One(t1) => if t1 > 0.0 { Some(t1) } else { None },
	// 		EqSolution::Two(t1, t2) => {
	// 			let min = f32::min(t1, t2);
	// 			let max = f32::max(t1, t2);
	// 			if min < 0.0 {
	// 				if max < 0.0 {
	// 					None
	// 				} else {
	// 					Some(max)
	// 				}
	// 			} else {
	// 				Some(min)
	// 			}
	// 		}
	// 	}
	// }

	fn get_closest_hit(&self, ray: Ray) -> Option<f32> {
		let L = self.center - ray.origin;
		let tca = L.dot(ray.direction);
		let d2 = L.dot(L) - tca * tca;
		if d2 > self.radius * self.radius {
			None
		} else {
			let thc = f32::sqrt(self.radius * self.radius - d2);
			let t0 = tca - thc;
			let t1 = tca + thc;
			let max = f32::max(t0, t1);
			let min = f32::min(t0, t1);

			if min < 0.0 {
				if max < 0.0 {
					None
				} else {
					Some(max)
				}
			} else {
				Some(min)
			}
		}
	}
}

pub struct Triangle {
	pub v1: Vertex,
	pub v2: Vertex,
	pub v3: Vertex
}

impl Triangle {
	pub fn new(v1: Vertex, v2: Vertex, v3: Vertex) -> Triangle {
		Triangle { v1: v1, v2: v2, v3: v3 }
	}

	// TODO: move to Shape
	pub fn aabb(&self) -> AABB {
		// FIXME
		AABB { min: Vec3f::new(0.0, 0.0, 0.0), max: Vec3f::new(0.0, 0.0, 0.0) }
	}

	fn get_barycentric(&self, p: Vec3f) -> Point2f {
		let v0 = self.v2.position - self.v1.position;
		let v1 = self.v3.position - self.v1.position;
		let v2 = p - self.v1.position;

		let d00 = v0.dot(v0);
		let d01 = v0.dot(v1);
		let d11 = v1.dot(v1);
		let d20 = v2.dot(v0);
		let d21 = v2.dot(v1);
		let denom = d00 * d11 - d01 * d01;
		let v = (d11 * d20 - d01 * d21) / denom;
		let w = (d00 * d21 - d01 * d20) / denom;
		let u = 1.0 - v - w;
		Point2f::new(u, v)
	}
}


impl Shape for Triangle {
	fn normal(&self, point: Vec3f) -> Vec3f {
		// TODO: interpolate the vertex normals
		let uv = self.get_barycentric(point);
		let norm = (1.0 - uv.x - uv.y) * self.v1.normal + uv.x * self.v2.normal + uv.y * self.v3.normal;
		norm.normalize()
		// (self.v2.position - self.v1.position).cross(self.v3.position - self.v1.position).normalize()
	}

	fn get_closest_hit(&self, ray: Ray) -> Option<f32> {
		let eps = 0.0000001;
		let e1 = self.v2.position - self.v1.position;
		let e2 = self.v3.position - self.v1.position;
		let h = ray.direction.cross(e2);
		let a = e1.dot(h);
		if a > -eps && a < eps {
			None
		} else {
			let f = 1.0 / a;
			let s = ray.origin - self.v1.position;
			let u = f * s.dot(h);
			if u < 0.0 || u > 1.0 {
				None
			} else {
				let q = s.cross(e1);
				let v = f * ray.direction.dot(q);
				if v < 0.0 || u + v > 1.0 {
					None
				} else {
					let t = f * e2.dot(q);
					if t > eps {
						Some(t)
					} else {
						None
					}
				}
			}
		}
	}
}

// #[test]
// fn test_triangle_normal() {
// 	let v1 = vec3(0.0, 0.0, 0.0);
// 	let v2 = vec3(1.0, 0.0, 0.0);
// 	let v3 = vec3(0.0, 1.0, 0.0);
// 	let triangle = Triangle { v1: v1, v2: v2, v3: v3 };
// 	let n = triangle.normal(vec3(0.0, 0.0, 0.0));
// 	assert_eq!(n.x, 0.0);
// 	assert_eq!(n.y, 0.0);
// 	assert_eq!(n.z, 1.0);
// }