mod game;
mod main_menu;

use bevy::prelude::*;
use game::*;
use main_menu::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .run()
}
