use macroquad::prelude::*;
const SCROLL_SENSITIVITY:f32 = 0.1;
pub struct Camera {
    pub x:f32,
    pub y:f32,
    pub zoom:f32,
}

pub fn update_camera(c: &mut Camera){
    if is_key_down(KeyCode::I){
        c.y -= 1.0;
    }
    if is_key_down(KeyCode::J){
        c.x -= 1.0;
    }
    if is_key_down(KeyCode::K){
        c.y += 1.0;
    }
    if is_key_down(KeyCode::L){
        c.x += 1.0;
    }
    c.zoom *= 1.0 + (SCROLL_SENSITIVITY * mouse_wheel().1)
}

pub fn world_to_screen(x: f32, y: f32, c: Camera) -> (f32, f32) {

    let screen_x = (x - c.x) * c.zoom + screen_width() / 2.0;
    let screen_y = (y - c.y) * c.zoom + screen_height() / 2.0;

    (screen_x, screen_y)
}
