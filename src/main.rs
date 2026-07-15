mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::draw_polygon;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::BLACK);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    framebuffer.set_current_color(Color::RED);

    let hexagon = vec![
        Vector2::new(300.0, 100.0),
        Vector2::new(400.0, 150.0),
        Vector2::new(400.0, 250.0),
        Vector2::new(300.0, 300.0),
        Vector2::new(200.0, 250.0),
        Vector2::new(200.0, 150.0),
    ];

    draw_polygon(&mut framebuffer, &hexagon);

    let output_file = "polygon.png";
    framebuffer.render_to_file(output_file);

    println!("Polígono guardado exitosamente!");
}