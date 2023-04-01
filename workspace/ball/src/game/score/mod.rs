use bevy::prelude::*;

pub mod comp;
mod system;

use crate::AppState;

use self::comp::*;
use self::system::*;

use super::InGameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<HighScores>()
            .add_system(init_score.in_schedule(OnEnter(AppState::Game)))
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)))
            .add_systems(
                (update_score, update_high_scores, high_scores_update)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(InGameState::Running)),
            );
    }
}
