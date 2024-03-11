use crate::math::{
    number::Number,
    vector::{
        accessor::{Accessor2d, Accessor3d},
        Vector2, Vector3,
    },
};
use std::convert::TryInto;

pub struct Input<const T: usize> {
    pub pos: Vector3,
    pub varying: [f64; T],
}

pub struct Output<const T: usize> {
    pub pos: Vector2<usize>,
    pub depth: f64,
    pub varying: [f64; T],
}

pub fn rasterize<const T: usize>(
    vertices: &[Input<T>; 3],
    width: usize,
    height: usize,
) -> Vec<Output<T>> {
    let mut fragements = Vec::<Output<T>>::new();

    let depth = Vector3::new(
        vertices[0].pos.z(),
        vertices[1].pos.z(),
        vertices[2].pos.z(),
    );

    let varying: Vec<Vector3<f64>> = (0..T)
        .map(|index| {
            Vector3::new(
                vertices[0].varying[index],
                vertices[1].varying[index],
                vertices[2].varying[index],
            )
        })
        .collect();

    let vertices: Vec<Vector2> = vertices
        .iter()
        .map(|vertex| {
            Vector2::new(
                (vertex.pos.x() * 0.5 + 0.5) * width as f64,
                (vertex.pos.y() * 0.5 + 0.5) * height as f64,
            )
        })
        .collect();

    let area = edge(&vertices[0], &vertices[1], &vertices[2]);

    for i in 0..width {
        for j in 0..height {
            let point = Vector2::new(i as f64 + 0.5, j as f64 + 0.5);

            let mut weights = [
                edge(&vertices[1], &vertices[2], &point),
                edge(&vertices[2], &vertices[0], &point),
                edge(&vertices[0], &vertices[1], &point),
            ];

            if weights.iter().any(|weight| *weight > 0.0) {
                continue;
            };

            weights.iter_mut().for_each(|weight| *weight /= area);

            let weight = Vector3::new(weights[0], weights[1], weights[2]);

            fragements.push(Output {
                pos: Vector2::new(i, j),
                depth: depth.dot(&weight),
                varying: varying
                    .iter()
                    .map(|v| v.dot(&weight))
                    .collect::<Vec<f64>>()
                    .try_into()
                    .unwrap(),
            })
        }
    }

    fragements
}

#[inline]
fn edge<V1, V2, V3, T>(a: &V1, b: &V2, c: &V3) -> T
where
    T: Number,
    V1: Accessor2d<T>,
    V2: Accessor2d<T>,
    V3: Accessor2d<T>,
{
    (c.x() - a.x()) * (b.y() - a.y()) - (c.y() - a.y()) * (b.x() - a.x())
}
