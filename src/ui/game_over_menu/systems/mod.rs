mod interactions;
mod layout;

use bevy::prelude::*;

use crate::AppState;

use self::{
    interactions::{
        interact_with_main_menu_button, interact_with_quit_button, interact_with_restart_button,
    },
    layout::{despawn_game_over_menu, spawn_game_over_menu},
};

pub struct GameOverMenuPlugin;
impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State
            .add_system(spawn_game_over_menu.in_schedule(OnEnter(AppState::GameOver)))
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
