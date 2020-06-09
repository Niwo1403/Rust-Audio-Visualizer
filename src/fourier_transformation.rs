use dft::{Operation, Plan, c64};

pub fn transform<Complex>(mut data: &mut Vec<Complex<f64>>) -> &mut Vec<Complex<f64>> {
    let plan = Plan::new(Operation::Forward, 512);
    dft::transform(&mut data, &plan);
    return &mut data;
}

pub fn example_vec<Complex>() -> &mut Vec<Complex<f64>> {
    let mut v = vec![c64::new(42.0, 69.0); 512];
    return &mut v;
}