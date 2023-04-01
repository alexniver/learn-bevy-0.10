use bevy::prelude::{IntoSystemConfigs, Plugin};

pub mod comp;
pub mod consts;
mod system;

use self::system::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_player)
            .add_systems((player_movement, confine_player_movement).chain())
            .add_systems((enemy_hit_player, player_hit_star).chain());
    }
}
