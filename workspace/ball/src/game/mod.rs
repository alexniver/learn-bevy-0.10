use bevy::prelude::*;

pub mod event;
mod system;

mod enemy;
mod player;
mod score;
mod star;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use system::*;

use self::{event::GameOver, score::comp::Score};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Score>()
            .add_event::<GameOver>()
            .add_startup_system(spawn_camera)
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_system(exit_game)
            .add_system(handle_game_over);
    }
}
