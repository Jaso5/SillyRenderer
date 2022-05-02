

#[derive(Copy, Clone)]
pub struct TexCoord {
    pub tex_coord: [f32; 2],
}

implement_vertex!(TexCoord, tex_coord);