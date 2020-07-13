#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub trait Drawable {
    //get shape for drawing with LinesList
    fn get_shape_outline(&self) -> Vec<Vertex>;
    //get shape for drawing with TrianglesList
    fn get_shape_fill(&self) -> Vec<Vertex>;
}