
use glium::{glutin, Surface, Frame, Display, VertexBuffer};
use std::time::{SystemTime, UNIX_EPOCH};
use std::f32::consts::PI;
use std::collections::LinkedList;
use super::rect::{Rect, drawable};


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);


pub struct freq_bars{
    list_rects: Vec<Rect>,
}

impl freq_bars{
    pub fn init_frequency_bars() -> freq_bars{
        let mut freq_bars = freq_bars{ list_rects: vec![]};

        for i in 0..20 {
            let width = 2.0;
            let offset = 1.0;

            let x = (0.0-offset)+i as f32*(width/20 as f32)+0.005;
            let y = 0.0;
            let width = (width/20 as f32)-0.01;
            let height = -0.2;

            let rect = Rect::new(x, y, width, height);

            freq_bars.list_rects.push(rect);

        }
        return freq_bars;
    }

    pub fn draw_visualizer(&mut self, mut target: Frame, display: &Display){
        let vertex1 = Vertex {position: [-0.4, 0.0]};
        let vertex2 = Vertex {position: [-0.3, 0.0]};
        let vertex3 = Vertex {position: [-0.3, 0.4]};
        let vertex4 = Vertex {position: [-0.4, 0.4]};

        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();




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


        //let rectTest = Rect::new([-0.4, 0.0], [-0.1, 0.0], [-0.1, 0.4], [-0.4, 0.4]);
        //target.draw(&rectTest.get_vertex_buffer(display), &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();


        target.finish().unwrap();




    }


}