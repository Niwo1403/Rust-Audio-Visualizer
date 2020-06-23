use glium::{glutin, Surface, Frame, Display, VertexBuffer};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Rect{
    c1: [f32; 2],
    c2: [f32; 2],
    c3: [f32; 2],
    c4: [f32; 2],
    redraw: bool,
}

pub trait drawable {
    fn update(&mut self);
    fn get_vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex>;
    fn new(c1 : [f32; 2], c2 : [f32; 2], c3 : [f32; 2], c4 : [f32; 2]) -> Rect;
    fn redraw(&mut self) -> bool;
}

impl drawable for Rect{
    fn update(&mut self){
        self.redraw = true;
    }
    fn get_vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex>{
        let vertex1 = Vertex {position: [self.c1[0], self.c1[1]]};
        let vertex2 = Vertex {position: [self.c1[0], self.c1[1]]};
        let vertex3 = Vertex {position: [self.c1[0], self.c1[1]]};
        let vertex4 = Vertex {position: [self.c1[0], self.c1[1]]};

        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        return vertex_buffer;
    }
    fn new(c1 : [f32; 2], c2 : [f32; 2], c3 : [f32; 2], c4 : [f32; 2]) -> Rect{
        return Rect {c1, c2, c3, c4, redraw: true };
    }
    fn redraw(&mut self) -> bool{
        if self.redraw{
            self.redraw = false;
            return true;
        }else {
            return false;
        }
    }
}