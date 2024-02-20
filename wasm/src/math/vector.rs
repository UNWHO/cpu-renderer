use super::matrix::Matrix;

pub type Vector<T, const S: usize> = Matrix<T, S, 1>;

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

pub mod accessor;

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
