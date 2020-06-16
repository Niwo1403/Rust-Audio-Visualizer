
use glium::{glutin, Surface, Frame, Display};
use std::time::{SystemTime, UNIX_EPOCH};

use super::Frequency_Bars;

pub fn draw(mut target: Frame,  display: &Display, startTime: &SystemTime) {

    Frequency_Bars::drawVisualizer(target, display, startTime);

}