pub mod attribute;
pub mod buffer;
pub mod rasterize;
pub mod shader;
pub mod texture;
pub mod uniform;

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
