use super::color::Color;
use crate::math::vector::Vector3;

pub struct Vertex {
    pub pos: Vector3<f64>,
    pub color: Color,
}
