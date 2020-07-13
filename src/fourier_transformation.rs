use dft::{Operation, Plan, c64};

pub static FFT_LENGTH : usize = 4096;


pub fn compute_fft(data: Vec<f32>) -> Vec<f32>{
    let c64_values = data_to_c64(data);
    let transformed_values = transform(c64_values);

    let mut scaled_bin_array = vec![0 as f32; FFT_LENGTH/2 as usize];
    let mut square1: f32;
    let mut square2: f32;
    let one_over_log_ten = 1.0/(10.0 as f32).ln();
    for k in 0..(FFT_LENGTH/2) as usize{

        square1 = (transformed_values[k].re * transformed_values[k].re) as f32;
        square2 = (transformed_values[k].im * transformed_values[k].im) as f32;

        scaled_bin_array[k] = 10.0 * (  (1.0 + (square1+square2)*(one_over_log_ten) ).ln() );

    }

    return scaled_bin_array;
}

fn transform(mut data: Vec<c64>) -> Vec<c64> {
    let plan = Plan::new(Operation::Forward, FFT_LENGTH);
    dft::transform(&mut data, &plan);
    return data;
}

pub fn apply_window(fft_window_length: usize, mut samples: Vec<f32>, window_coefficients: &Vec<f32>) -> Vec<f32>{
    let mut sample: f32;
    for i in 0..fft_window_length {
        sample = samples[i] * window_coefficients[i];
        samples[i] = sample;
    }

    return samples;
}

pub fn data_to_c64(byte_data: Vec<f32>) -> Vec<c64> {
    let data = byte_data.into_iter()
        .map( |number| {c64::new(number as f64, 0.0)})
        .collect::<Vec<c64>>();
    return data;
}
