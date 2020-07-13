use crate::drawable::Vertex;
use crate::drawable::Drawable;

pub struct Line{
    pub x_start: f32,
    pub y_start: f32,
    pub x_end: f32,
    pub y_end: f32,
}

impl Line{

    pub fn new(x_start : f32, y_start : f32, x_end : f32, y_end : f32) -> Line{
        return Line {x_start, y_start, x_end, y_end};
    }

    pub fn set_x_start(&mut self, x_start: f32){
        self.x_start = x_start;
    }

    pub fn set_y_start(&mut self, y_start: f32){
        self.y_start = y_start;
    }

    pub fn set_x_end(&mut self, x_end: f32){
        self.x_end = x_end;
    }

    pub fn set_y_end(&mut self, y_end: f32){
        self.y_end = y_end;
    }

}

impl Drawable for Line{
    fn get_shape_outline(&self) -> Vec<Vertex>{
        let vertex1 = Vertex {position: [self.x_start, self.y_start]};
        let vertex2 = Vertex {position: [self.x_end, self.y_end]};

        let shape = vec![vertex1, vertex2];

        return shape;
    }

    fn get_shape_fill(&self) -> Vec<Vertex>{
        let vertex1 = Vertex {position: [self.x_start, self.y_start]};
        let vertex2 = Vertex {position: [self.x_end, self.y_end]};

        let shape = vec![vertex1, vertex2, vertex1];

        return shape;
    }
}