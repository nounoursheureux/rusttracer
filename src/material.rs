use image::Color;

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub opaque: bool,
    pub reflective: bool
}