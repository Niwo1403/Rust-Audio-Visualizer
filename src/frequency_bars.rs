
use glium::{Surface, Frame, Display, Program, index::NoIndices};
use std::f32::consts::PI;
use std::collections::VecDeque;
use crate::rect::Rect;
use crate::fourier_transformation;
use crate::fourier_transformation::FFT_LENGTH;
use std::sync::mpsc::Receiver;
use crate::drawable::Drawable;
use crate::visualizer::Visualizer;


static GAIN : f32 = 2.4;
static SLOPE : f32 = 0.01;
static NUM_RECTS : i64 = 256;

pub struct FreqBars {
    list_rects: Vec<Rect>,
    value_list: VecDeque<f32>,
    window_coefficients: Vec<f32>,
    program: Program,
    indices: NoIndices,
}

impl Visualizer for FreqBars {
    fn init_visualizer(display: &Display) -> FreqBars {

        //init fftMaths
        let mut window_coefficients = vec![0 as f32; FFT_LENGTH];
        for i in 0..FFT_LENGTH {
            window_coefficients[i] = 0.54 - 0.46*(2.0*PI*i as f32/FFT_LENGTH as f32).cos();
        }

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


        let mut freq_bars = FreqBars { list_rects: vec![], value_list: VecDeque::with_capacity(FFT_LENGTH), window_coefficients, program: program, indices: indices};
        //init rectangles
        for i in 0..NUM_RECTS{
            let width = 2.0;
            let offset = 1.0;

            let x = (0.0-offset)+i as f32*(width/NUM_RECTS as f32)+0.005;
            let y = -0.95;
            let width = (width/NUM_RECTS as f32)-0.01;
            let height = -0.2;

            let rect = Rect::new(x, y, width, height);

            freq_bars.list_rects.push(rect);

        }
        //set all values to 0
        for _i in 0..FFT_LENGTH {
            freq_bars.value_list.push_front(0 as f32);
        }

        return freq_bars;
    }

    fn draw_visualizer(&mut self, mut target: Frame, display: &Display){

        //value_list zu Vec<f32>
        let value_slices = self.value_list.as_slices();
        let mut fourier_input = value_slices.0.to_vec();
        fourier_input.append(&mut value_slices.1.to_vec());

        //apply_window nutzen um Daten zu "verschönern"
        fourier_input = fourier_transformation::apply_window(FFT_LENGTH, fourier_input, &self.window_coefficients);

        //fft anwenden
        let mut fourier_output = fourier_transformation::compute_fft(fourier_input);

        //falls array zu lang - auf anzahl der Balken skalieren
        if NUM_RECTS < (FFT_LENGTH/2) as i64 {
            fourier_output = scale_array_down(fourier_output, NUM_RECTS as usize);
        }

        //Höhen der Balken setzen
        let mut i = 0;
        for value in fourier_output.iter_mut(){
            let y = ( (GAIN + SLOPE*i as f32)*(*value)) / 200.0 as f32;
            //println!("Y: {}", y);  // max: 0.6889446
            self.list_rects[i].set_height(y);

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
        self.value_list.push_back(value);
        while self.value_list.len() > FFT_LENGTH {
            self.value_list.pop_front();
        }
    }


}

fn scale_array_down(data: Vec<f32>, new_length: usize) -> Vec<f32>{
    let mut new_vec = vec![0 as f32; new_length];
    let mut temp: f32;
    for i in 0..new_length{
        /*let windowWidth = (data.len() as f32 / new_length as f32);
        let startAvgWindow = (i as f32*windowWidth - windowWidth/2.0);
        let endAvgWindow = startAvgWindow+windowWidth;
        let mut nums = 0.0;
        for j in startAvgWindow as usize..endAvgWindow as usize{
            temp += data[j];
            nums += 1.0;
        }
        temp = temp/nums;*/
        temp = data[i * (data.len() as f32 / new_length as f32).round() as usize];
        new_vec[i] = temp;
    }

    return new_vec;
}


