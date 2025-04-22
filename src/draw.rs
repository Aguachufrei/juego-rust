use macroquad::prelude::*;
use crate::screen;
pub fn rectangle(x: f32, y: f32, w:f32, h:f32, col:Color, cam:&screen::Camera){
    let (sx, sy) = screen::world_to_screen(x, y, &cam);
    draw_rectangle(sx, sy, w*cam.zoom, h*cam.zoom, col);
}
