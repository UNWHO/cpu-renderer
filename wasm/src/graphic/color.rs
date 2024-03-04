use crate::math::vector::Vector4;

pub type Color = Vector4<f64>;

impl Color {
    pub fn white() -> Self {
        Vector4::const_from([[1.0, 1.0, 1.0, 1.0]])
    }

    pub fn black() -> Self {
        Vector4::const_from([[0.0, 0.0, 0.0, 1.0]])
    }

    pub fn red() -> Self {
        Vector4::const_from([[1.0, 0.0, 0.0, 1.0]])
    }

    pub fn green() -> Self {
        Vector4::const_from([[0.0, 1.0, 0.0, 1.0]])
    }

    pub fn blue() -> Self {
        Vector4::const_from([[0.0, 0.0, 1.0, 1.0]])
    }
}
