use bevy::prelude::{IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin};

pub mod comp;
pub mod consts;
mod system;

use crate::AppState;

use self::system::*;

use super::InGameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))
            .add_systems(
                (player_movement, confine_player_movement)
                    .chain()
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(InGameState::Running)),
            )
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .chain()
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(InGameState::Running)),
            );
    }
}
