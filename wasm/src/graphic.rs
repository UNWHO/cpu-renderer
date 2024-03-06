pub mod attribute;
pub mod buffer;
pub mod color;
pub mod fragment;
pub mod uniform;
pub mod vertex;

use self::{color::Color, fragment::Fragment, vertex::Vertex};
use crate::math::{
    matrix::Matrix,
    vector::{Vector2, Vector3},
};

pub fn vertex_shader(vertex: &Vertex, mvp_matrix: &Matrix<f64, 4, 4>) -> Vertex {
    Vertex {
        pos: vertex
            .pos
            .to_homogeneous()
            .mul_matrix(mvp_matrix)
            .from_homogeneous(),
        color: vertex.color.clone(),
    }
}

pub fn rasterize(triangle: &Vec<Vertex>, width: usize, height: usize) -> Vec<Fragment> {
    let mut fragements = Vec::<Fragment>::new();

    let vertices: Vec<Vector2<f64>> = triangle
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
            let depth = Vector3::new(
                triangle[0].pos.z(),
                triangle[1].pos.z(),
                triangle[2].pos.z(),
            );
            let red = Vector3::new(
                triangle[0].color.x(),
                triangle[1].color.x(),
                triangle[2].color.x(),
            );
            let green = Vector3::new(
                triangle[0].color.y(),
                triangle[1].color.y(),
                triangle[2].color.y(),
            );
            let blue = Vector3::new(
                triangle[0].color.z(),
                triangle[1].color.z(),
                triangle[2].color.z(),
            );
            let alpha = Vector3::new(
                triangle[0].color.w(),
                triangle[1].color.w(),
                triangle[2].color.w(),
            );

            fragements.push(Fragment {
                pos: Vector2::new(i, j),
                depth: depth.dot(&weight),
                color: Color::new(
                    red.dot(&weight),
                    green.dot(&weight),
                    blue.dot(&weight),
                    alpha.dot(&weight),
                ),
            })
        }
    }

    fragements
}

#[inline]
fn edge(a: &Vector2<f64>, b: &Vector2<f64>, c: &Vector2<f64>) -> f64 {
    (c.x() - a.x()) * (b.y() - a.y()) - (c.y() - a.y()) * (b.x() - a.x())
}

// pub fn render(width: usize, height: usize, buffer: &mut Vec<u8>) {
//     for i in 0..width {
//         for j in 0..height {
//             let color = COLORS[(j * width + i) % 3];

//             buffer[j * width * 4 + i * 4] = color.x();
//             buffer[j * width * 4 + i * 4 + 1] = color.y();
//             buffer[j * width * 4 + i * 4 + 2] = color.z();
//             buffer[j * width * 4 + i * 4 + 3] = color.w();
//         }
//     }
// }
