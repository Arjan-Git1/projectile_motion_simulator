use std::{ffi::FromBytesUntilNulError, fmt::format};

use macroquad::prelude::*;
fn Configuration()->Conf {
    Conf { window_title: "Projectile Motion Simulator".to_string(), fullscreen: true, ..Default::default()}
}
enum State {
    Menu,
    Choose,
    Simulate
}
#[macroquad::main(Configuration)]
async fn main() {
    let angled :f32= 60.0;
    let angle = angled.to_radians();
    let mut u =500.0;
    let mut g = 980.0;
    let mut x = 32.0;
    let mut y = screen_height()/2.0;
    let start = x;
        let vertical_motion = u*angle.sin();
        let  start_y = y;
    
    let  mut t = 0.0;

 let mut  state = State::Menu;
    loop {
         
         clear_background(DARKBROWN);
                 match state {
             State::Menu =>{
             draw_text("PRESS SPACE TO GET STARTED", screen_width()/2.0 - 10.0, screen_height()/2.0, 50.0, RED);
             if is_key_pressed(KeyCode::Space) {
                 state = State::Choose;
             }
                            }
            State::Choose =>{
                 draw_text("G is Fixed at 9.8", 0.0, screen_height()/2.0, 50.0, RED);
                 let text  = format!(" Initial Velocity = {}" , u);
                 draw_text("Press the  up key to increase and dwon to decrease u.", 0.0, 75.0, 50.0, RED);
                 draw_text(text.as_str(), screen_width()/2.0, 32.0, 50.0, RED);
                 if is_key_pressed(KeyCode::Up) {
                     u+=10.0;
                     }
                 if is_key_pressed(KeyCode::Space) {
                    state = State::Simulate;
                    t= 0.0;
                    x=start;
                    y =start_y;
                 } 
                 
                           }
            State::Simulate=>{
                t = t + get_frame_time();
                 let range = (u * u * (2.0 * angle).sin()) / g;
               if x<start + range {
                   x = start + u*angle.cos()*t;
                   y = start_y - (u*angle.sin()*t - 0.5*g*t*t);
               }
               draw_circle(x, y, 32.0, RED);
            }
                    
         }
         
         next_frame().await;
    }
}
