pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::HighScores;
use systems::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)))
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_systems((update_high_scores, high_scores_updated));
    }
}
