pub mod event;
mod game;
mod main_menu;
mod system;

use bevy::prelude::*;
use event::*;
use game::*;
use main_menu::*;
use system::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_event::<GameOver>()
        .add_startup_system(spawn_camera)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run()
}

#[derive(States, Default, Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
