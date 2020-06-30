use glium::{glutin, Surface, Frame, Display, VertexBuffer};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Rect{
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    redraw: bool,
}

pub trait drawable {
    fn update(&mut self);
    fn get_vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex>;
    fn new(x : f32, y : f32, width : f32, height : f32) -> Rect;
    fn redraw(&mut self) -> bool;
    fn set_height(&mut self, height: f32);
}

impl drawable for Rect{
    fn update(&mut self){
        self.redraw = true;
    }
    fn get_vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex>{
        let vertex1 = Vertex {position: [self.x, self.y]};
        let vertex2 = Vertex {position: [self.x, self.y+self.height]};
        let vertex3 = Vertex {position: [self.x+self.width, self.y+self.height]};
        let vertex4 = Vertex {position: [self.x+self.width, self.y]};

        let shape = vec![vertex1, vertex2, vertex3, vertex4];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        return vertex_buffer;
    }
    fn new(x : f32, y : f32, width : f32, height : f32) -> Rect{
        return Rect {x, y, width, height, redraw: true };
    }
    fn redraw(&mut self) -> bool{
        if self.redraw{
            self.redraw = false;
            return true;
        }else {
            return false;
        }
    }
    fn set_height(&mut self, height: f32){
        self.height = height;
        self.update();
    }
}