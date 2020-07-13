
use glium::{glutin, Surface};
use glutin::dpi::LogicalSize;

use crate::frequency_bars::FreqBars;
use std::sync::mpsc::Receiver;
use crate::visualizer::Visualizer;
use crate::oscilloscope::Oscilloscope;
use crate::waveform::Waveform;
use crate::drawable::Drawable;


pub fn start_window(value_receiver: Receiver<f32>, visualizer_type: i32){
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Aduio Visualizer").with_inner_size(LogicalSize::new(600, 300));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut target = display.draw();
    target.clear_color(1.0, 1.0, 1.0, 1.0);
    target.finish().unwrap();

    //Visualizer initialisieren
    let mut draw_method_1 = FreqBars::init_visualizer(&display);
    let mut draw_method_2 = Oscilloscope::init_visualizer(&display);
    let mut draw_method_3 = Waveform::init_visualizer(&display);

    //loop
    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        //Daten vom Player bekommen
        let mut rec_result = value_receiver.try_recv();
        while rec_result.is_ok() {
            let new_val = rec_result.unwrap();
            draw_method_1.add_value(new_val);
            draw_method_2.add_value(new_val);
            draw_method_3.add_value(new_val);
            rec_result = value_receiver.try_recv();
        }


        //----------------draw visualizer--------------------------
        let target = display.draw();

        if visualizer_type == 1 {
            draw_method_1.draw_visualizer(target, &display);
        } else if visualizer_type == 2 {
            draw_method_2.draw_visualizer(target, &display);
        } else if visualizer_type == 3 {
            draw_method_3.draw_visualizer(target, &display);
        }
        //---------------end drawing------------------------------

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });

}
