use crate::game::SimulationState;
use crate::ui::interact_with_button_changes_color;
use crate::ui::pause_menu::components::*;
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn interact_with_resume_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        interact_with_button_changes_color(&interaction, &mut background_color);
        if *interaction == Interaction::Clicked {
            simulation_state_next_state.set(SimulationState::Running);
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
            // A bit hacky: leaving the paused state will thus despawn the pause menu
            simulation_state_next_state.set(SimulationState::Running);
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
