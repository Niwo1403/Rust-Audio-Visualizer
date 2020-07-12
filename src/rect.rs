#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Rect{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    old_height: f32,
    redraw: bool,
    pub shape: Vec<Vertex>,
}

pub trait Drawable {
    fn update(&mut self);
    fn update_shape(&mut self);
    fn new(x : f32, y : f32, width : f32, height : f32) -> Rect;
    fn redraw(&mut self) -> bool;
    fn set_height(&mut self, height: f32);
    fn get_old_height(&self) -> f32;
}

impl Drawable for Rect{
    fn update(&mut self){
        self.update_shape();
        self.redraw = true;
    }
    fn update_shape(&mut self){
        let vertex1 = Vertex {position: [self.x, self.y]};
        let vertex2 = Vertex {position: [self.x, self.y+self.height]};
        let vertex3 = Vertex {position: [self.x+self.width, self.y+self.height]};
        let vertex4 = Vertex {position: [self.x+self.width, self.y]};

        let shape = vec![vertex1, vertex2, vertex2, vertex3, vertex3, vertex4, vertex4, vertex1];

        //uncomment for line-drawMethode
        self.shape = shape;
    }
    fn new(x : f32, y : f32, width : f32, height : f32) -> Rect{
        let vertex1 = Vertex {position: [x, y]};
        let vertex2 = Vertex {position: [x, y+height]};
        let vertex3 = Vertex {position: [x+width, y+height]};
        let vertex4 = Vertex {position: [x+width, y]};

        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        return Rect {x, y, width, height, old_height: 0.0, redraw: true, shape: shape};
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
        if self.height != height {
            self.old_height = self.height;
            self.height = height;
            self.update();
        }

    }
    fn get_old_height(&self) -> f32{
        return self.old_height;
    }
}