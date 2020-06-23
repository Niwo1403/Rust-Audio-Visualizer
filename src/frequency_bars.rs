
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



static mut LIST_RECTS: LinkedList<Rect> = LinkedList::new();

pub fn init_frequency_bars(){


    for i in 0..20 {
        let width = 1.0;
        let offset = 0.5;

        //let rect = Rect {c1: [i/21]}
        //let rect= Rect::new()
        let rect = Rect::new([-0.4, 0.0], [-0.3, 0.0], [-0.3, 0.4], [-0.4, 0.4]);

        unsafe{
            LIST_RECTS.push_back(rect);
        }
    }





}

pub fn draw_visualizer(mut target: Frame, display: &Display, start_time: &SystemTime){

    /*#[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);
*/
    /*let ellapsed = match startTime.elapsed(){
        Ok(ellapsed) => ellapsed.as_secs_f32(),
        Err(error) => 0f32
    };


    //10/6

    let vertex1 = Vertex {position: [0.5*(PI/2f32+ellapsed).cos(), 0.5*(PI/2f32+ellapsed).sin()] };
    //let vertex2 = Vertex { position: [ 0.5*(PI/2f32+PI/3f32).cos(),  0.5*(PI/2f32+PI/3f32).sin()] };
    let vertex2 = Vertex { position: [ 0.5*(PI+PI/6f32+ellapsed).cos(),  0.5*(PI+PI/6f32+ellapsed).sin()] };
    let vertex3 = Vertex { position: [ 0.5*(-PI/6f32+ellapsed).cos(), 0.5*(-PI/6f32+ellapsed).sin()] };
    /*let radius = 1f32ei als Argument übergeben.

Process finished with exit code 1;
    let vertex1 = Vertex {position: [(ellapsed+PI/3f32).cos()*radius, (ellapsed+PI/3f32).sin()*radius] };
    let vertex2 = Vertex {position: [(ellapsed+2f32*(PI/3f32)).cos()*radius, (ellapsed+2f32*(PI/3f32)).sin()*radius] };
    let vertex3 = Vertex {position: [(ellapsed).cos()*radius, (ellapsed).sin()*radius] };*/

    let shape = vec![vertex1, vertex2, vertex3];*/

    //let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

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
    //------------------------------------------------------------------------



    target.clear_color(1.0, 1.0, 1.0, 1.0);


    /*unsafe {
        let mut i = 0;
        for rect in LIST_RECTS.iter_mut() {
            if(rect.redraw()){
                println!("redrawing: {}", i);
                target.draw(&rect.get_vertex_buffer(display), &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            }
            i += 1;
        }
    }*/


    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();


    target.finish().unwrap();




}