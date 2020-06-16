
use glium::{glutin, Surface, Frame, Display};
use std::time::{SystemTime, UNIX_EPOCH};

use super::frequency_bars;

pub fn draw(mut target: Frame, display: &Display, start_time: &SystemTime) {

    frequency_bars::draw_visualizer(target, display, start_time);

}