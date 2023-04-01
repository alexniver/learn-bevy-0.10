use bevy::prelude::*;

pub mod comp;
pub mod consts;
mod system;

use crate::AppState;

use self::comp::*;
use self::system::*;

use super::InGameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(spawn_enemy.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)))
            .add_systems(
                (
                    update_enemy_timer,
                    spawn_enemy_by_timer,
                    enemy_move,
                    update_enemy_dir,
                    confine_enemy_movement,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(InGameState::Running)),
            );
    }
}
