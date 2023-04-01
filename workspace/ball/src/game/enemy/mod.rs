use bevy::prelude::*;

pub mod comp;
pub mod consts;
mod system;

use self::comp::*;
use self::system::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_startup_system(spawn_enemy)
            .add_system(update_enemy_timer)
            .add_system(spawn_enemy_by_timer)
            .add_system(enemy_move)
            .add_system(update_enemy_dir)
            .add_system(confine_enemy_movement);
    }
}
