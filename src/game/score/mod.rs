pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::{HighScores, Score};
use systems::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_system(reset_score.in_schedule(OnExit(AppState::Game)))
            .add_systems(
                (update_high_scores, high_scores_updated).in_set(OnUpdate(AppState::GameOver)),
            );
    }
}
