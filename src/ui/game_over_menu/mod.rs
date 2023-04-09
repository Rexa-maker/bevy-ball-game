mod components;
mod systems;

use self::systems::interactions::*;
use self::systems::layout::*;
use crate::AppState;
use bevy::prelude::*;

pub struct GameOverMenuPlugin;
impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Listens to GameOver event within Game state
            .add_system(spawn_game_over_menu.in_set(OnUpdate(AppState::Game)))
            // Systems
            .add_systems(
                (
                    interact_with_restart_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                )
                    .in_set(OnUpdate(AppState::GameOver)),
            )
            // OnExit State
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GameOver)));
    }
}
