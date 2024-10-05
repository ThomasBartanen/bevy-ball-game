use bevy::prelude::*;
use rand::prelude::*;

pub fn get_random_screen_point(window: &Window) -> Vec3 {
    let random_x: f32 = random::<f32>() * window.width();
    let random_y: f32 = random::<f32>() * window.height();

    Vec3 {
        x: random_x,
        y: random_y,
        z: 0.0,
    }
}

pub fn randomize_choice() -> bool {
    random::<f32>() >= 0.5
}

pub fn get_random_edge_point(window: &Window) -> Vec3 {
    let edge: f32 = random::<f32>();
    let mut x: f32 = random::<f32>();
    let mut y: f32 = random::<f32>();

    if edge >= 0.5 {
        // Sides
        if x >= 0.5 {
            x = window.width() - 1.0
        } else {
            x = 1.0
        };

        y *= window.height();
    } else {
        // Top/Bottom
        if y >= 0.5 {
            y = window.height() - 1.0
        } else {
            y = 1.0
        };

        x *= window.width();
    }

    Vec3 { x, y, z: 0.0 }
}
