use crate::line::Line;
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use glium::{Program, Display, Frame, Surface};
use glium::index::NoIndices;
use crate::visualizer::Visualizer;
use crate::drawable::Drawable;
use crate::rect::Rect;


static NUM_VALUES : i64 = 1024;
static AVG_COUNT: i64 = 512;

pub struct Waveform {
    list_rects: Vec<Rect>,
    value_list: VecDeque<f32>,
    avg_values: Vec<f32>,
    program: Program,
    indices: NoIndices,
}

impl Visualizer for Waveform{
    fn init_visualizer(display: &Display) -> Waveform {

        //init drawing
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
        "#;

        let fragment_shader_src = "
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
        ";

        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();


        let mut waveform = Waveform { list_rects: vec![], value_list: VecDeque::with_capacity(NUM_VALUES as usize), avg_values: vec![], program: program, indices: indices};
        for i in 0..NUM_VALUES{
            let width = 2.0;
            let offset = 1.0;

            let x_start = (0.0-offset)+i as f32*(width/NUM_VALUES as f32);
            let y_start = 0.0;
            let x_end = (0.0-offset)+(i+1) as f32*(width/NUM_VALUES as f32);
            let y_end = 0.0;

            let rect = Rect::new(x_start, y_start, x_end-x_start, y_end-y_start);

            waveform.list_rects.push(rect);

            waveform.value_list.push_front(0 as f32);
        }

        return waveform;
    }

    fn draw_visualizer(&mut self, mut target: Frame, display: &Display){

        //value_list zu Vec<f32>
        let value_slices = self.value_list.as_slices();
        let mut valueVec = value_slices.0.to_vec();
        valueVec.append(&mut value_slices.1.to_vec());

        //Line y werte setzen
        let mut i = 0;
        for value in valueVec.iter_mut(){
            let y = (*value) / 2.0 as f32;
            //println!("Y: {}", y);  // max: 0.6889446
            self.list_rects[i].set_height(y*2.0);
            self.list_rects[i].set_y(0.0-y);
            i += 1;
        }


        target.clear_color(1.0, 1.0, 1.0, 1.0);

        //combine rectangles and draw
        let mut shapes = vec![];
        for rect in self.list_rects.iter_mut() {
            let mut rect_shape = rect.get_shape_fill();
            shapes.append(&mut rect_shape);

            i += 1;
        }

        let vertex_buffer = glium::VertexBuffer::new(display, &shapes).unwrap();
        target.draw(&vertex_buffer, &self.indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();

    }

    fn add_value(&mut self, value: f32){
        self.avg_values.push(value.abs());

        if self.avg_values.len() == AVG_COUNT as usize {
            let mut sum = 0.0;

            //combine by avg
            for val in self.avg_values.iter(){
                sum += *val;
            }
            sum /= AVG_COUNT as f32;

            //combine by max
            let mut max = 0.0;
            for val in self.avg_values.iter(){
                if *val > max {max = *val;}
            }

            self.value_list.push_back(max);
            self.avg_values.clear();
        }

        while self.value_list.len() > NUM_VALUES as usize {
            self.value_list.pop_front();
        }
    }

}