pub mod enemy;
pub mod events;
mod player;
pub mod score;
pub mod star;
mod systems;

use bevy::prelude::*;
use rand::random;

use super::AppState;
use enemy::EnemyPlugin;
use events::GameOver;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)))
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

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
