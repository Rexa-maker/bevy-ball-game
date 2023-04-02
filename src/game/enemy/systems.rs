use super::components::*;
use super::resources::*;
use super::{ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES};
use crate::utils::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

fn spawn_single_enemy(commands: &mut Commands, window: &Window, asset_server: &Res<AssetServer>) {
    let mut random_vec = get_random_2d_position(window);
    confine_sprite(ENEMY_SIZE, window, &mut random_vec);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(random_vec).with_scale(Vec3::new(0.5, 0.5, 0.0)),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        },
        Enemy {
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
        },
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        spawn_single_enemy(&mut commands, window, &asset_server)
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;

        if translation.x < x_min {
            enemy.direction.x = enemy.direction.x.abs();
            direction_changed = true;
        } else if translation.x > x_max {
            enemy.direction.x = -1.0 * enemy.direction.x.abs();
            direction_changed = true;
        }

        if translation.y < y_min {
            enemy.direction.y = enemy.direction.y.abs();
            direction_changed = true;
        } else if translation.y > y_max {
            enemy.direction.y = -1.0 * enemy.direction.y.abs();
            direction_changed = true;
        }

        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
            let sound_effect = if random::<bool>() {
                sound_effect_1
            } else {
                sound_effect_2
            };
            audio.play(sound_effect);
        }
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        spawn_single_enemy(&mut commands, window, &asset_server);
    }
}
