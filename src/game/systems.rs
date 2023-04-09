use super::SimulationState;
use crate::AppState;
use bevy::prelude::*;

pub fn pause_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Paused);
    println!("Simulation paused.");
}

pub fn resume_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Running);
    println!("Simulation resumed.");
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: ResMut<State<SimulationState>>,
    next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.0 {
            SimulationState::Running => {
                pause_simulation(next_simulation_state);
            }
            SimulationState::Paused => {
                resume_simulation(next_simulation_state);
            }
        }
    }
}
