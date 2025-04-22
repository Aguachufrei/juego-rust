use macroquad::prelude::*;

const NONE:u8 = 0;
const LOW:u8 = 1;
const HIGH:u8 = 2;

pub struct Directions {
    pub up: u8,
    pub down: u8,
    pub left: u8,
    pub right: u8,
}


pub fn check_keys(dir: &mut Directions){
    
    //W↑
    //Mira si la tecla esta siendo pulsada
    if is_key_down(KeyCode::W)||is_key_down(KeyCode::Up){
        //Mira si la tecla estaba desacrivada y la activa
        //ESTE IF NO ES REDUNDANTE
        //METERLO EN LA CONDICION DE ARRIBA MODIFICA LA CLAUSULA ELSE
        if dir.up == NONE {
            dir.up = HIGH;
            //Baja la prioridad a la tecla contraria
            if dir.down == HIGH{
                dir.down = LOW;
            }
        }
    } else {
        //finalmente se desactiva si se deja de pulsar
        dir.up = NONE;
    }

    //A← 
    if is_key_down(KeyCode::A)||is_key_down(KeyCode::Left){
        if dir.left == NONE {
            dir.left = HIGH;
            if dir.right == HIGH{
                dir.right = LOW;
            }
        }
    } else {
        dir.left = NONE;
    }

    //S↓
    if is_key_down(KeyCode::S)||is_key_down(KeyCode::Down){
        if dir.down == NONE {
            dir.down = HIGH;
            if dir.up == HIGH{
                dir.up = LOW;
            }
        }   
    } else {
        dir.down = NONE;
    }

    //D→
    if is_key_down(KeyCode::D)||is_key_down(KeyCode::Right){
        if dir.right == NONE {
            dir.right = HIGH;
            if dir.left == HIGH {
                dir.left = LOW; 
            }
        }
    } else {
        dir.right = NONE;
    }
}
