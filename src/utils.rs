use bevy::prelude::*;
use rand::prelude::*;

pub fn confine_sprite(size: f32, window: &Window, translation: &mut Vec3) {
    let half_size = size / 2.0;
    let x_min = 0.0 + half_size;
    let x_max = window.width() - half_size;
    let y_min = 0.0 + half_size;
    let y_max = window.height() - half_size;

    if translation.x < x_min {
        translation.x = x_min;
    } else if translation.x > x_max {
        translation.x = x_max;
    }

    if translation.y < y_min {
        translation.y = y_min;
    } else if translation.y > y_max {
        translation.y = y_max;
    }
}

pub fn get_random_2d_position(window: &Window) -> Vec3 {
    Vec3::new(
        random::<f32>() * window.width(),
        random::<f32>() * window.height(),
        0.0,
    )
}
