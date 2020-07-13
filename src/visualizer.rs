use std::sync::mpsc::Receiver;
use glium::{Frame, Display};

pub trait Visualizer {
    fn init_visualizer(display: &Display) -> Self;
    fn draw_visualizer(&mut self, target: Frame, display: &Display);
    fn add_value(&mut self, value: f32);
}