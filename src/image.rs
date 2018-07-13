extern crate png;
extern crate cgmath;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use self::png::HasParameters;
use self::png::EncodingError;
use types::Vec3f;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

impl Color {
	pub fn new(r: f32, g: f32, b: f32) -> Color {
		Color { r: r, g: g, b: b }
	}

	pub fn from_vec(v: Vec3f) -> Color {
		Color { r: v.x, g: v.y, b: v.z }
	}

	pub fn black() -> Color {
		Color { r: 0.0, g: 0.0, b: 0.0 }
	}
}

impl Add for Color {
	type Output = Color;

	fn add(self, other: Color) -> Color {
		Color {
			r: f32::min(self.r + other.r, 1.0),
			g: f32::min(self.g + other.g, 1.0),
			b: f32::min(self.b + other.b, 1.0) 
		}
	}
}

impl AddAssign for Color {
	fn add_assign(&mut self, other: Color) {
		*self = *self + other;
	}
}


pub struct Image {
	pub width: u32,
	pub height: u32,
	pub data: Vec<Color>
}

impl Image {
	pub fn write(&self, path: &Path) -> Result<(), EncodingError> {
		let file = File::create(path).unwrap();
		let ref mut w = BufWriter::new(file);

		let mut encoder = png::Encoder::new(w, self.width, self.height);
		encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
		let mut writer = encoder.write_header().unwrap();

		let mut data: Vec<u8> = Vec::new();
		for col in self.data.iter() {
			// gamma correction
			data.push((col.r.powf(1.0/2.2) * 255.0) as u8);
			data.push((col.g.powf(1.0/2.2) * 255.0) as u8);
			data.push((col.b.powf(1.0/2.2) * 255.0) as u8);
		}

		writer.write_image_data(&data)
	}
}