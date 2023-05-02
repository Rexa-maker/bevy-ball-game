pub mod components;
pub mod systems;

use self::systems::interactions::*;
use self::systems::layout::*;
use crate::game::SimulationState;
use crate::AppState;
use bevy::prelude::*;

pub struct PauseMenuPlugin;
impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter
            .add_system(
                spawn_pause_menu
                    .in_schedule(OnEnter(SimulationState::Paused))
                    .in_set(OnUpdate(AppState::Game)),
            )
            // Systems
            .add_systems(
                (
                    interact_with_resume_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                )
                    .in_set(OnUpdate(SimulationState::Paused)),
            )
            // OnExit
            .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)));
    }
}
