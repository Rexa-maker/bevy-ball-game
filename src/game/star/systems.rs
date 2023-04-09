use super::components::Star;
use super::resources::StarSpawnTimer;
use super::{NUMBER_OF_STARS, STAR_SIZE};
use crate::game::{confine_sprite, get_random_2d_position};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn spawn_single_star(commands: &mut Commands, window: &Window, asset_server: &Res<AssetServer>) {
    let mut random_vec = get_random_2d_position(window);
    confine_sprite(STAR_SIZE, window, &mut random_vec);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(random_vec).with_scale(Vec3::new(0.5, 0.5, 0.0)),
            texture: asset_server.load("sprites/star.png"),
            ..default()
        },
        Star {},
    ));
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        spawn_single_star(&mut commands, window, &asset_server);
    }
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for star in star_query.iter() {
        commands.entity(star).despawn();
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        spawn_single_star(&mut commands, window, &asset_server);
    }
}
