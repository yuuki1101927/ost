#[derive(Copy, Clone)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 2],
}

implement_vertex!(Vertex, position);