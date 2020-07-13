use crate::line::Line;
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use glium::{Program, Display, Frame, Surface};
use glium::index::NoIndices;
use crate::visualizer::Visualizer;
use crate::drawable::Drawable;


static NUM_VALUES : i64 = 4096;

pub struct Oscilloscope {
    list_lines: Vec<Line>,
    value_list: VecDeque<f32>,
    program: Program,
    indices: NoIndices,
}

impl Visualizer for Oscilloscope{
    fn init_visualizer(display: &Display) -> Oscilloscope {


        //init drawing
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

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


        let mut oscilloscope = Oscilloscope { list_lines: vec![], value_list: VecDeque::with_capacity(NUM_VALUES as usize), program: program, indices: indices};
        for i in 0..NUM_VALUES{
            let width = 2.0;
            let offset = 1.0;

            let x_start = (0.0-offset)+i as f32*(width/NUM_VALUES as f32);
            let y_start = 0.0;
            let x_end = (0.0-offset)+(i+1) as f32*(width/NUM_VALUES as f32);
            let y_end = 0.0;

            let line = Line::new(x_start, y_start, x_end, y_end);

            oscilloscope.list_lines.push(line);

            oscilloscope.value_list.push_front(0 as f32);
        }

        return oscilloscope;
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
            self.list_lines[i].set_y_end(y);
            self.list_lines[i+1].set_y_start(y);
            i += 1;

            if i+1 >= NUM_VALUES as usize {break;}
        }


        target.clear_color(1.0, 1.0, 1.0, 1.0);

        let mut shapes = vec![];

        for line in self.list_lines.iter_mut() {
            let mut line_shape = line.get_shape_outline();
            shapes.append(&mut line_shape);

            i += 1;
        }

        let vertex_buffer = glium::VertexBuffer::new(display, &shapes).unwrap();
        target.draw(&vertex_buffer, &self.indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();

    }

    fn add_value(&mut self, value: f32){
        self.value_list.push_back(value);
        while self.value_list.len() > NUM_VALUES as usize {
            self.value_list.pop_front();
        }
    }

}