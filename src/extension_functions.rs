use bevy::prelude::*;
use rand::prelude::*;


pub fn get_random_screen_point(    
    window: &Window
) -> Vec3 {
    let random_x: f32 = random::<f32>() * window.width();
    let random_y: f32 = random::<f32>() * window.height();    
    
    return Vec3 { x: random_x, y: random_y, z: 0.0 };
}

pub fn get_random_edge_point(
    window: &Window
) -> Vec3 {
    let mut x: f32 = random::<f32>();
    let mut y: f32 = random::<f32>();
    if x >= 0.5 { x = window.width() + 0.1 };
    if y >= 0.5 { y = window.height() + 0.1 };
    
    Vec3 { x: x, y: y, z: 0.0 }
}