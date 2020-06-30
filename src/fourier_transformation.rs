use dft::{Operation, Plan, c64};

pub fn transform(mut data: Vec<c64>) -> Vec<c64> {
    let plan = Plan::new(Operation::Forward, 512);
    dft::transform(&mut data, &plan);
    return data;
}

pub fn data_to_c64(byte_data: Vec<f32>) -> Vec<c64> {
    let data = byte_data.into_iter()
        .map( |number| {c64::new(number as f64, 0.0)})
        .collect::<Vec<c64>>();
    return data;
}

fn thread_example() {
    // Thread Test
    let t1 = std::thread::spawn(move || {
        for i in 1..1000 {
            if i%100 == 0 {
                println!("1");
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    });
    let t2 = std::thread::spawn(move || {
        for i in 1..1000 {
            if i%100 == 0 {
                println!("2");
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    });
    t1.join();
    t2.join();
}