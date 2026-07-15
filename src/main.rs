mod framebuffer;

use raylib::prelude::*;
use framebuffer::Framebuffer;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    framebuffer.set_current_color(Color::WHITE);
    framebuffer.set_pixel(400, 300);

    framebuffer.render_to_file("framebuffer_test.png");

    println!("Framebuffer guardado exitosamente!");
}