use glium::{glutin, Surface, Frame, Display, VertexBuffer};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Rect{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    old_height: f32,
    redraw: bool,
    vertexBuffer: VertexBuffer<Vertex>,
}

pub trait drawable {
    fn update(&mut self);
    fn update_vertex_buffer(&mut self, display: &Display) -> &VertexBuffer<Vertex>;
    fn get_vertex_buffer(&self) -> &VertexBuffer<Vertex>;
    fn new(x : f32, y : f32, width : f32, height : f32, display: &Display) -> Rect;
    fn redraw(&mut self) -> bool;
    fn set_height(&mut self, height: f32);
    fn get_old_height(&self) -> f32;
}

impl drawable for Rect{
    fn update(&mut self){
        self.redraw = true;
    }
    fn update_vertex_buffer(&mut self, display: &Display) -> &VertexBuffer<Vertex>{
        let vertex1 = Vertex {position: [self.x, self.y]};
        let vertex2 = Vertex {position: [self.x, self.y+self.height]};
        let vertex3 = Vertex {position: [self.x+self.width, self.y+self.height]};
        let vertex4 = Vertex {position: [self.x+self.width, self.y]};

        let shape = vec![vertex1, vertex2, vertex3, vertex4];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        self.vertexBuffer = vertex_buffer;
        //println!("Updating VertexBuffer");

        return &self.vertexBuffer;
    }
    fn get_vertex_buffer(&self) -> &VertexBuffer<Vertex>{
        return &self.vertexBuffer;
    }
    fn new(x : f32, y : f32, width : f32, height : f32, display: &Display) -> Rect{
        let vertex1 = Vertex {position: [x, y]};
        let vertex2 = Vertex {position: [x, y+height]};
        let vertex3 = Vertex {position: [x+width, y+height]};
        let vertex4 = Vertex {position: [x+width, y]};

        let shape = vec![vertex1, vertex2, vertex3, vertex4];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();


        return Rect {x, y, width, height, old_height: 0.0, redraw: true, vertexBuffer: vertex_buffer};
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
        if(self.height != height) {
            self.old_height = self.height;
            self.height = height;
            self.update();
        }

    }
    fn get_old_height(&self) -> f32{
        return self.old_height;
    }
}