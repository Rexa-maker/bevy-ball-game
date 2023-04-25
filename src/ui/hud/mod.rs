mod components;
mod systems;

use self::systems::interactions::*;
use self::systems::layout::*;
use crate::AppState;
use bevy::prelude::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter
            .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems((update_score, update_enemies).in_set(OnUpdate(AppState::Game)))
            // OnExit
            .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
    }
}
