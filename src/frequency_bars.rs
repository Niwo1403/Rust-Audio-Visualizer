
use glium::{glutin, Surface, Frame, Display, VertexBuffer};
use std::time::{SystemTime, UNIX_EPOCH};
use std::f32::consts::PI;
use std::collections::{LinkedList, VecDeque};
use super::rect::{Rect, drawable};
use crate::fourier_transformation;
use std::sync::mpsc::Receiver;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);


pub struct freq_bars{
    list_rects: Vec<Rect>,
    valueList: VecDeque<f32>,
    rec: Receiver<f32>
}

impl freq_bars{
    pub fn init_frequency_bars(ValueReciver: Reciver<f32>) -> freq_bars{
        let numRects = 512;
        let mut freq_bars = freq_bars{ list_rects: vec![], valueList: VecDeque::with_capacity(numRects), rec: ValueReciver};
        for i in 0..numRects {
            let width = 2.0;
            let offset = 1.0;

            let x = (0.0-offset)+i as f32*(width/numRects as f32)+0.005;
            let y = 0.0;
            let width = (width/numRects as f32)-0.01;
            let height = -0.2;

            let rect = Rect::new(x, y, width, height);

            freq_bars.list_rects.push(rect);
            freq_bars.valueList.push_front(0 as f32);
        }
        return freq_bars;
    }

    pub fn draw_visualizer(&mut self, mut target: Frame, display: &Display){
        //Daten vom Player bekommen und mit fft umwandeln
        /*let newVal = self.rec.recv().unwrap();
        self.valueList.push_back(newVal);
        while(self.valueList.len() > 512){
            self.valueList.pop_front();
        }

        let mut valueSlices = self.valueList.as_slices();
        let mut fourierInput = valueSlices.0.to_vec();
        fourierInput.append(&mut valueSlices.1.to_vec());

        let mut c64Values = fourier_transformation::data_to_c64(fourierInput);
        let mut transformedValues = fourier_transformation::transform(c64Values);
        let mut i = 0;
        for value in transformedValues.iter_mut(){
            self.list_rects[i].set_height(value.re as f32);


            i += 1;
        }*/

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);

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

        target.clear_color(1.0, 1.0, 1.0, 1.0);


        let mut i = 0;
        for rect in self.list_rects.iter_mut() {
            //if (rect.redraw()) {
                println!("redrawing: {}", i);
                let vertex_buffer = rect.get_vertex_buffer(display);
                target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            //}
            i += 1;
        }

        target.finish().unwrap();




    }


}