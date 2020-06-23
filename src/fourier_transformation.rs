use dft::{Operation, Plan, c64};

pub fn transform(mut data: Vec<c64>) -> Vec<c64> {
    let plan = Plan::new(Operation::Forward, 512);
    dft::transform(&mut data, &plan);
    return data;
}

pub fn data_to_c64(byte_data: Vec<i8>) -> Vec<c64> {
    let data = byte_data.into_iter()
        .map( |number| {c64::new(number as f64, 0.0)})
        .collect::<Vec<c64>>();
    return data;
}
