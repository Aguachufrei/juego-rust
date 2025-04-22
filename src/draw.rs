use macroquad::prelude::*;

pub fn draw_square(x: f32, y: f32) {
    draw_rectangle(x, y, 50.0, 50.0, BLUE);
}

pub fn draw_circle(x: f32, y: f32) {
    draw_circle(x, y, 25.0, RED);
}
