use bevy::prelude::IntoSystemAppConfig;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::OnUpdate;
use bevy::prelude::Plugin;

pub mod comp;
pub mod consts;
pub mod res;
mod system;

use crate::AppState;

use self::res::*;
use self::system::*;

use super::InGameState;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<StarSpawnTimer>()
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)))
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(InGameState::Running)),
            );
    }
}
