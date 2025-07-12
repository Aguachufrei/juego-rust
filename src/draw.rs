use macroquad::prelude::*;
use crate::screen;
pub fn rectangle(x: f32, y: f32, w:f32, h:f32, col:Color, &cam:screen::Camera){
    let (sx, sy) = screen::world_to_screen(x, y, cam);
    draw_rectangle(sx, sy, w*cam.zoom, h*cam.zoom, col);
}
pub fn fixed_rectangle(x: f32, y:f32, w:f32, h:f32, col:Color){
    draw_rectangle(x, y, w, h, col);
}
pub fn fixed_scaled_rectangle(x: f32, y:f32, w:f32, h:f32, is_x:bool, is_y:bool, col:Color){
    if is_x && is_y {
        draw_rectangle(x, y, w/100.0*screen_width(), h/100.0*screen_height(), col);
    } else if is_x {
        draw_rectangle(x, y, w/100.0*screen_width(), h, col);
    } else if is_y {
        draw_rectangle(x, y, w, h/100.0*screen_height(), col);
    } else {
        eprintln!("No zoom aplied, if this is intentional, use scaled_rectangle instead");
        fixed_rectangle(x, y, w, h, col);
    }
}
pub fn create_grid(x:f32, y:f32, cols:i32, rows:i32, w:f32, h:f32, t:f32, col:Color, cam:&screen::Camera){
    let at:f32 = t*cam.zoom;
    if at<1.0 {return};
    for i in 0..=cols {
        let (sx1, sy1) = screen::world_to_screen(x+w*(i as f32), y, cam);
        let (sx2, sy2) = screen::world_to_screen(x+w*(i as f32), y+h*(rows as f32), cam);
        draw_line(sx1, sy1, sx2, sy2, at, col);
    }
    for j in 0..=rows {
        let (sx1, sy1) = screen::world_to_screen(x, y+h*(j as f32), cam);
        let (sx2, sy2) = screen::world_to_screen(x+w*(cols as f32), y+h*(j as f32), cam);
        draw_line(sx1, sy1, sx2, sy2, at, col);
    }
}
//pub fn image (x:f32, y:f32, w:f32, h:f32, image:Image, cam:screen::Camera)
