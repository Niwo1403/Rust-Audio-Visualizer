
use glium::{glutin, Surface, Frame, Display, VertexBuffer};
use std::time::{SystemTime, UNIX_EPOCH};
use std::f32::consts::PI;
use std::collections::{LinkedList, VecDeque};
use super::rect::{Rect, drawable};
use crate::fourier_transformation;
use std::sync::mpsc::Receiver;
use dft::c64;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

static GAIN : f32 = 2.4;
static SLOPE : f32 = 0.01;
static NUM_RECTS : i64 = 256;

pub struct freq_bars{
    list_rects: Vec<Rect>,
    valueList: VecDeque<f32>,
    rec: Receiver<f32>,
    windowCoefficients: Vec<f32>,
}

impl freq_bars{
    pub fn init_frequency_bars(ValueReceiver: Receiver<f32>) -> freq_bars{
        let fftWindowLength = (NUM_RECTS*2) as usize;
        let mut windowCoefficients = vec![0 as f32; fftWindowLength];
        for i in 0..fftWindowLength {
            windowCoefficients[i] = 0.54 - 0.46*(2.0*PI*i as f32/fftWindowLength as f32).cos();
        }

        let mut freq_bars = freq_bars{ list_rects: vec![], valueList: VecDeque::with_capacity((NUM_RECTS*2) as usize), rec: ValueReceiver, windowCoefficients: windowCoefficients};
        for i in 0..NUM_RECTS{
            let width = 2.0;
            let offset = 1.0;

            let x = (0.0-offset)+i as f32*(width/NUM_RECTS as f32)+0.005;
            let y = 0.0;
            let width = (width/NUM_RECTS as f32)-0.01;
            let height = -0.2;

            let rect = Rect::new(x, y, width, height);

            freq_bars.list_rects.push(rect);

        }

        for i in 0..(NUM_RECTS*2) {
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
            println!("Rec new val: {}", newVal);
            self.valueList.push_back(newVal);
            while(self.valueList.len() > (NUM_RECTS*2) as usize){
                self.valueList.pop_front();
            }
            recResult = self.rec.try_recv();
        }


        let mut valueSlices = self.valueList.as_slices();
        let mut fourierInput = valueSlices.0.to_vec();
        fourierInput.append(&mut valueSlices.1.to_vec());
        fourierInput = self.applyWindow((NUM_RECTS * 2) as usize, fourierInput);

        let mut fourierOutput = computeFFT(fourierInput);


        let mut i = 0;
        for value in fourierOutput.iter_mut(){
            let y = (( (GAIN + SLOPE*i as f32)*(*value)) / 500.0 as f32);
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
                //println!("redrawing: {}", i);
                let vertex_buffer = rect.get_vertex_buffer(display);
                target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            //}
            i += 1;
        }

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



fn computeFFT(data: Vec<f32>) -> Vec<f32>{
    let mut c64Values = fourier_transformation::data_to_c64(data);
    let mut transformedValues = fourier_transformation::transform(c64Values);

    let mut scaledBinArray = vec![0 as f32; (NUM_RECTS) as usize];
    let mut square1 = 0 as f32;
    let mut square2 = 0 as f32;
    let oneOverLogTen = 1.0/(10.0 as f32).ln();
    for k in 0..(NUM_RECTS) as usize{

        square1 = (transformedValues[k].re * transformedValues[k].re) as f32;
        square2 = (transformedValues[k].im * transformedValues[k].im) as f32;

        scaledBinArray[k] = 10.0 * (  (1.0 + (square1+square2)*(oneOverLogTen) ).ln() );

    }

    return scaledBinArray;
}