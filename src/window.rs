
use glium::{glutin, Surface, Frame, Display};
use glutin::dpi::LogicalSize;

use super::draw;
use std::time::SystemTime;

pub fn openWindow(){

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Aduio Visualizer").with_inner_size(LogicalSize::new(600, 600));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let startTime = SystemTime::now();

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        //----------------draw something--------------------------

        let mut target = display.draw();
        draw::draw(target, &display, &startTime);

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