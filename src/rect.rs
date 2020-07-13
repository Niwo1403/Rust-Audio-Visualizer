use crate::drawable::Vertex;
use crate::drawable::Drawable;

pub struct Rect{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect{

    pub fn new(x : f32, y : f32, width : f32, height : f32) -> Rect{
        return Rect {x, y, width, height};
    }

    pub fn set_height(&mut self, height: f32){
        self.height = height;
    }

    pub fn set_width(&mut self, width: f32){
        self.width = width;
    }

    pub fn set_x(&mut self, x: f32){
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32){
        self.y = y;
    }

}

impl Drawable for Rect{
    fn get_shape_outline(&self) -> Vec<Vertex>{
        let vertex1 = Vertex {position: [self.x, self.y]};
        let vertex2 = Vertex {position: [self.x, self.y+self.height]};
        let vertex3 = Vertex {position: [self.x+self.width, self.y+self.height]};
        let vertex4 = Vertex {position: [self.x+self.width, self.y]};

        let shape = vec![vertex1, vertex2, vertex2, vertex3, vertex3, vertex4, vertex4, vertex1];

        return shape;
    }

    fn get_shape_fill(&self) -> Vec<Vertex>{
        let vertex1 = Vertex {position: [self.x, self.y]};
        let vertex2 = Vertex {position: [self.x, self.y+self.height]};
        let vertex3 = Vertex {position: [self.x+self.width, self.y+self.height]};
        let vertex4 = Vertex {position: [self.x+self.width, self.y]};

        let shape = vec![vertex1, vertex2, vertex3, vertex1, vertex4, vertex3];

        return shape;
    }
}