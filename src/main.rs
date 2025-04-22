use macroquad::prelude::*;
mod keys;

///Constants
const SPEED : f32 = 200.0;
const NONE:u8 = 0;
const LOW:u8 = 1;
const HIGH:u8 = 2;
    
#[macroquad::main("Prueba Juego")]


async fn main() {
    let mut x = 40.0;
    let mut y = 40.0;
    
    let mut dir = keys::Directions { 
        up: NONE,
        down: NONE,
        left: NONE,
        right: NONE,
    };

    loop {
        clear_background(YELLOW);
        let dt = get_frame_time();
        
        keys::check_keys(& mut dir);
        movement(&mut x, &mut y, dt, &dir);
        

        draw_rectangle(x, y, 40.0, 40.0, BLACK);
        next_frame().await
    }
}
fn movement(x: &mut f32, y: &mut f32, dt: f32, dir: &keys::Directions){
    //mirar si necesitamos una reduccion por ir en diagonal
    let mut d: f32 = 0.707; //sqrt(2)
    if (dir.up + dir.down) == 0 || (dir.left + dir.right) == 0 {
       d = 1.0;
    }
    //EJE Y
    if dir.up == HIGH {
        *y -= SPEED*dt*d;
    } else if dir.down != NONE {
        *y += SPEED*dt*d; 
    } else if dir.up == LOW {
        *y -= SPEED*dt*d;
    }

    //EJE X
    if dir.left == HIGH {
        *x -= SPEED*dt*d;
    } else if dir.right != NONE {
        *x += SPEED*dt*d;
    } else if dir.left == LOW {
        *x -= SPEED*dt*d;
    }
}

fn QQcollision(x1:f32, y1:f32, l1:f32, x2:f32, y2:f32, l2:f32) -> bool{
    //TODO
    return true;
}
