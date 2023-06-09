use bevy::{app::AppExit, prelude::*};

use crate::{
    game::SimulationState,
    ui::{
        interact_with_button_changes_color,
        main_menu::components::{PlayButton, QuitButton},
    },
    AppState,
};

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        interact_with_button_changes_color(&interaction, &mut background_color);
        if *interaction == Interaction::Clicked {
            app_state_next_state.set(AppState::Game);
            simulation_state_next_state.set(SimulationState::Paused);
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        interact_with_button_changes_color(&interaction, &mut background_color);
        if *interaction == Interaction::Clicked {
            app_exit_event_writer.send(AppExit);
        }
    }
}
