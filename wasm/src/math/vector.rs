use super::{matrix::Matrix, number::Number};

pub type Vector<T, const S: usize> = Matrix<T, S, 1>;

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

pub mod accessor;

impl<T: Number, const S: usize> Vector<T, S> {
    pub fn dot(&self, rhs: &Self) -> T {
        let mut sum: T = T::zero();

        for i in 0..S {
            sum += self[0][i] * rhs[0][i]
        }

        sum
    }
}

impl<T: Number> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self::from(&[[x, y]])
    }
}

impl<T: Number> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self::from(&[[x, y, z]])
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::from(&[[
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        ]])
    }
}

impl<T: Number> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self::from(&[[x, y, z, w]])
    }
}

#[cfg(test)]
mod test {

    use super::Vector3;

    #[test]
    fn operate() {
        let mut vec1 = Vector3::from(&[[1, 2, 3]]);
        let vec2 = Vector3::from(&[[4, 5, 6]]);

        vec1.add(&vec2);
        assert_eq!(Vector3::from(&[[5, 7, 9]]), vec1)
    }
}
