
use glium::{glutin, Surface, Frame, Display};
use glutin::dpi::LogicalSize;

use std::time::SystemTime;
use crate::frequency_bars::freq_bars;



pub struct window_struct{
    draw_methode: freq_bars,

}

impl window_struct{
    pub fn open_window() -> window_struct{

        let mut drawMethode = freq_bars::init_frequency_bars();
        let window = window_struct {draw_methode: drawMethode};
        return window;
    }

    /*pub fn start_window(&mut self){
        let mut event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new().with_title("Aduio Visualizer").with_inner_size(LogicalSize::new(600, 600));
        let cb = glutin::ContextBuilder::new();
        let mut display = glium::Display::new(wb, cb, &event_loop).unwrap();
        let mut display_ref = &mut display;

        event_loop.run( move |ev, _, control_flow| {
            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);
            //----------------draw something--------------------------

            let mut target = display_ref.draw();
            self.draw_methode.draw_visualizer(target, display_ref);

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

    }*/

}

pub fn start_window(ValueReciver: Reciver<f32>){
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Aduio Visualizer").with_inner_size(LogicalSize::new(600, 600));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut target = display.draw();
    target.clear_color(1.0, 1.0, 1.0, 1.0);
    target.finish().unwrap();

    let mut drawMethode = freq_bars::init_frequency_bars(ValueReciver);
    //loop
    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        //----------------draw something--------------------------

        let mut target = display.draw();
        //draw::draw(target, &display, &start_time, &mut drawMethode);
        drawMethode.draw_visualizer(target, &display);

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
