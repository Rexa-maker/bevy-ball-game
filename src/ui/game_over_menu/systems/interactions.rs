use crate::game::SimulationState;
use crate::ui::game_over_menu::components::*;
use crate::ui::interact_with_button_changes_color;
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn interact_with_restart_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RestartButton>),
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

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        interact_with_button_changes_color(&interaction, &mut background_color);
        if *interaction == Interaction::Clicked {
            app_state_next_state.set(AppState::MainMenu);
            simulation_state_next_state.set(SimulationState::Paused);
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        interact_with_button_changes_color(&interaction, &mut background_color);
        if *interaction == Interaction::Clicked {
            app_exit_event_writer.send(AppExit);
        }
    }
}
