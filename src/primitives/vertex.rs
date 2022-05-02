#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3]) -> Self {
        Self { position }
    }
}

// #[derive(Copy, Clone)]
// pub struct Vertexture {
//     position: [f32; 3],
//     tex_coords: [f32; 2]
// }
//
// impl Vertexture {
//     pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Self {
//         Self {
//             position,
//             tex_coords,
//         }
//     }
// }

// #[derive(Copy, Clone)]
// pub struct Vertexture {
//     position: [f32; 3],
//     tex_coords: [f32; 2],
//     normal: [f32; 3],
// }
//
// impl Vertexture {
//     pub fn new(x: f32, y: f32, z: f32, tx: f32, ty: f32, nx: f32, ny: f32, nz: f32) -> Self {
//         Self {
//             position: [x, y, z],
//             tex_coords: [tx, ty],
//             normal: [nx, ny, nz]
//         }
//     }
// }

implement_vertex!(Vertex, position);
// implement_vertex!(Vertexture, position, tex_coords);
// implement_vertex!(FVert, position, tex_coords, normal);

