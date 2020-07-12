
use glium::{glutin, Surface};
use glutin::dpi::LogicalSize;

use crate::frequency_bars::FreqBars;
use std::sync::mpsc::Receiver;


pub fn start_window(value_receiver: Receiver<f32>){
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Aduio Visualizer").with_inner_size(LogicalSize::new(600, 300));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut target = display.draw();
    target.clear_color(1.0, 1.0, 1.0, 1.0);
    target.finish().unwrap();

    let mut draw_method = FreqBars::init_frequency_bars(value_receiver, &display);
    //loop
    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        //----------------draw something--------------------------
        let target = display.draw();
        draw_method.draw_visualizer(target, &display);
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
