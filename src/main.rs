
use raylib::prelude::*;

fn main() {
    let mut raylib = raylib::init();

    let (mut rayhandle, rthread) = raylib.title("Test").size(600, 600).resizable().build();

    while !rayhandle.window_should_close() {
        let mut rdraw = rayhandle.begin_drawing(&rthread);
        rdraw.clear_background(Color::BLACK);

        rdraw.draw_text("Hello from raylib-rs! and Ramen!", (rdraw.get_screen_width() / 2) - 150, rdraw.get_screen_height() / 2, 20, Color::DARKPURPLE);
    }
}
