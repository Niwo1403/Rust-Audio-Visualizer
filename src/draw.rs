
use glium::{glutin, Surface, Frame, Display};
use std::time::{SystemTime, UNIX_EPOCH};
use std::f32::consts::PI;


pub fn draw(mut target: Frame,  display: &Display, startTime: &SystemTime) {

    //---------------------------------new copy-------------------------------
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let ellapsed = match startTime.elapsed(){
        Ok(ellapsed) => ellapsed.as_secs_f32(),
        Err(error) => 0f32
    };


    //10/6

    println!("draw: {}", ellapsed);
    let vertex1 = Vertex {position: [0.5*(PI/2f32+ellapsed).cos(), 0.5*(PI/2f32+ellapsed).sin()] };
    //let vertex2 = Vertex { position: [ 0.5*(PI/2f32+PI/3f32).cos(),  0.5*(PI/2f32+PI/3f32).sin()] };
    let vertex2 = Vertex { position: [ 0.5*(PI+PI/6f32+ellapsed).cos(),  0.5*(PI+PI/6f32+ellapsed).sin()] };
    let vertex3 = Vertex { position: [ 0.5*(-PI/6f32+ellapsed).cos(), 0.5*(-PI/6f32+ellapsed).sin()] };
    /*let radius = 1f32;
    let vertex1 = Vertex {position: [(ellapsed+PI/3f32).cos()*radius, (ellapsed+PI/3f32).sin()*radius] };
    let vertex2 = Vertex {position: [(ellapsed+2f32*(PI/3f32)).cos()*radius, (ellapsed+2f32*(PI/3f32)).sin()*radius] };
    let vertex3 = Vertex {position: [(ellapsed).cos()*radius, (ellapsed).sin()*radius] };*/

    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let red = ellapsed.sin();
    let green = (ellapsed+PI/2f32).sin();
    let blue = (ellapsed+3f32*PI/2f32).sin();

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
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
    target.finish().unwrap();

}