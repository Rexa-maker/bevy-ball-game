use bevy::prelude::*;

use crate::{game::SimulationState, ui::game_over_menu::components::RestartButton};

pub fn interact_with_restart_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RestartButton>),
    >,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    // Despawn all the entities
}

pub fn interact_with_main_menu_button() {}

pub fn interact_with_quit_button() {}
