use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub fn fill_polygon(framebuffer: &mut Framebuffer, polygons: &[&[Vector2]], fill_color: Color) {
    let mut edges: Vec<(Vector2, Vector2)> = Vec::new();

    for points in polygons {
        let count = points.len();
        if count < 2 {
            continue;
        }
        for i in 0..count {
            let start = points[i];
            let end = points[(i + 1) % count];
            edges.push((start, end));
        }
    }

    if edges.is_empty() {
        return;
    }

    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for (start, end) in &edges {
        min_y = min_y.min(start.y).min(end.y);
        max_y = max_y.max(start.y).max(end.y);
    }

    let y_start = min_y.floor() as i32;
    let y_end = max_y.ceil() as i32;

    framebuffer.set_current_color(fill_color);

    for y in y_start..=y_end {
        let y_f = y as f32 + 0.5;
        let mut intersections: Vec<f32> = Vec::new();

        for (start, end) in &edges {
            let (p1, p2) = (start, end);

            if p1.y == p2.y {
                continue;
            }

            let (lower, upper) = if p1.y < p2.y { (p1, p2) } else { (p2, p1) };

            if y_f >= lower.y && y_f < upper.y {
                let t = (y_f - lower.y) / (upper.y - lower.y);
                let x = lower.x + t * (upper.x - lower.x);
                intersections.push(x);
            }
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut i = 0;
        while i + 1 < intersections.len() {
            let x_start = intersections[i].round() as i32;
            let x_end = intersections[i + 1].round() as i32;

            for x in x_start..x_end {
                if x >= 0 && y >= 0 {
                    framebuffer.set_pixel(x as u32, y as u32);
                }
            }

            i += 2;
        }
    }
}