use bevy::prelude::Plugin;

pub mod comp;
pub mod consts;
pub mod res;
mod system;

use self::res::*;
use self::system::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<StarSpawnTimer>()
            .add_startup_system(spawn_stars)
            .add_system(tick_star_spawn_timer)
            .add_system(spawn_stars_over_time);
    }
}
