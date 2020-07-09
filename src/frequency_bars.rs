
use glium::{glutin, Surface, Frame, Display, VertexBuffer, Program, index::NoIndices};
use std::time::{SystemTime, UNIX_EPOCH};
use std::f32::consts::PI;
use std::collections::{LinkedList, VecDeque};
use super::rect::{Rect, drawable};
use crate::fourier_transformation;
use crate::fourier_transformation::FFT_LENGTH;
use crate::rect::Vertex;
use std::sync::mpsc::Receiver;
use dft::c64;


static GAIN : f32 = 2.4;
static SLOPE : f32 = 0.01;
static NUM_RECTS : i64 = 256;

pub struct freq_bars{
    list_rects: Vec<Rect>,
    valueList: VecDeque<f32>,
    rec: Receiver<f32>,
    windowCoefficients: Vec<f32>,
    program: Program,
    indices: NoIndices,
}

impl freq_bars{
    pub fn init_frequency_bars(ValueReceiver: Receiver<f32>, display: &Display) -> freq_bars{

        //init fftMaths
        let mut windowCoefficients = vec![0 as f32; FFT_LENGTH];
        for i in 0..FFT_LENGTH {
            windowCoefficients[i] = 0.54 - 0.46*(2.0*PI*i as f32/FFT_LENGTH as f32).cos();
        }

        //init drawing
        //let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
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


        let mut freq_bars = freq_bars{ list_rects: vec![], valueList: VecDeque::with_capacity(FFT_LENGTH), rec: ValueReceiver, windowCoefficients: windowCoefficients, program: program, indices: indices};
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

        for i in 0..FFT_LENGTH {
            freq_bars.valueList.push_front(0 as f32);
        }

        return freq_bars;
    }

    pub fn draw_visualizer(&mut self, mut target: Frame, display: &Display){
        //Daten vom Player bekommen und mit fft umwandeln


        /*let recResult = self.rec.try_recv();
        if(recResult.is_ok()){
            let newVal = recResult.unwrap();
            println!("Rec new val: {}", newVal);
            self.valueList.push_back(newVal);
            while(self.valueList.len() > (NUM_RECTS*2) as usize){
                self.valueList.pop_front();
            }
        }*/

        let mut recResult = self.rec.try_recv();
        while(recResult.is_ok()){
            let newVal = recResult.unwrap();
            self.valueList.push_back(newVal);
            while(self.valueList.len() > FFT_LENGTH){
                self.valueList.pop_front();
            }
            recResult = self.rec.try_recv();
        }


        let mut valueSlices = self.valueList.as_slices();
        let mut fourierInput = valueSlices.0.to_vec();
        fourierInput.append(&mut valueSlices.1.to_vec());
        fourierInput = self.applyWindow(FFT_LENGTH, fourierInput);

        let mut fourierOutput = computeFFT(fourierInput);
        if(NUM_RECTS < (FFT_LENGTH/2) as i64){
            fourierOutput = scaleArrayDown(fourierOutput, NUM_RECTS as usize);
        }
        let mut i = 0;
        for value in fourierOutput.iter_mut(){
            let y = (( (GAIN + SLOPE*i as f32)*(*value)) / 400.0 as f32);
            //println!("Y: {}", y);
            self.list_rects[i].set_height(y);


            i += 1;
        }

        /*for i in 0..NUM_RECTS{
            let value = transformedValues[i as usize];
            let y = ( (GAIN + SLOPE*i as f32)*value.re as f32) / 100.0;

            self.list_rects[i as usize].set_height(value.re as f32);
        }*/

        /*let mut i = 0;
        for value in fourierInput.iter(){
            self.list_rects[i].set_height(*value);


            i += 1;
        }*/



        target.clear_color(1.0, 1.0, 1.0, 1.0);


        let mut shapes = vec![];
        //uncomment for line-drawMethode
        //shapes.push(Vertex {position: [-1 as f32, 0 as f32]});

        let mut i = 0;
        for rect in self.list_rects.iter_mut() {
            /*if(rect.redraw()){
                rect.update_vertex_buffer(display);

            }*/
            let mut rect_shape = rect.shape.clone();
            shapes.append(&mut rect_shape);

            i += 1;
        }

        let vertex_buffer = glium::VertexBuffer::new(display, &shapes).unwrap();
        target.draw(&vertex_buffer, &self.indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();




    }

    fn applyWindow(&self, fftWindowlength: usize, mut samples: Vec<f32>) -> Vec<f32>{
        let mut sample = 0 as f32;
        for i in 0..fftWindowlength{
            sample = samples[i] * self.windowCoefficients[i];
                samples[i] = sample;
        }

        return samples;
    }


}

fn scaleArrayDown(data: Vec<f32>, new_length: usize) -> Vec<f32>{
    let mut new_vec = vec![0 as f32; new_length];
    let mut temp = 0 as f32;
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


fn computeFFT(data: Vec<f32>) -> Vec<f32>{
    let mut c64Values = fourier_transformation::data_to_c64(data);
    let mut transformedValues = fourier_transformation::transform(c64Values);

    let mut scaledBinArray = vec![0 as f32; FFT_LENGTH/2 as usize];
    let mut square1 = 0 as f32;
    let mut square2 = 0 as f32;
    let oneOverLogTen = 1.0/(10.0 as f32).ln();
    for k in 0..(FFT_LENGTH/2) as usize{

        square1 = (transformedValues[k].re * transformedValues[k].re) as f32;
        square2 = (transformedValues[k].im * transformedValues[k].im) as f32;

        scaledBinArray[k] = 10.0 * (  (1.0 + (square1+square2)*(oneOverLogTen) ).ln() );

    }

    return scaledBinArray;
}