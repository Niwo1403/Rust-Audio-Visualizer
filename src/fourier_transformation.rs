use dft::{Operation, Plan, c64};

pub fn transform(mut data: Vec<c64>) -> Vec<c64> {
    let plan = Plan::new(Operation::Forward, 512);
    dft::transform(&mut data, &plan);
    return data;
}

pub fn example_vec() -> Vec<c64> {
    let mut v = vec![c64::new(42.0, 69.0); 512];
    return v;
}