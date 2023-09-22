use bevy::prelude::*;
use rand::prelude::*;


pub fn get_random_screen_point(    
    window: &Window
) -> Vec3 {
    let random_x: f32 = random::<f32>() * window.width();
    let random_y: f32 = random::<f32>() * window.height();    
    
    return Vec3 { x: random_x, y: random_y, z: 0.0 };
}