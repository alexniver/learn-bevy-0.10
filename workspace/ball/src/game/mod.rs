use bevy::prelude::*;

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

use crate::AppState;

use self::score::comp::Score;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<InGameState>()
            .init_resource::<Score>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_system(toggle_in_game_state.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Default, Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum InGameState {
    #[default]
    Paused,
    Running,
}
