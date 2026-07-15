use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub fn draw_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    let count = points.len();

    if count < 2 {
        return;
    }

    for i in 0..count {
        let start = points[i];
        let end = points[(i + 1) % count];

        line(framebuffer, start, end);
    }
}