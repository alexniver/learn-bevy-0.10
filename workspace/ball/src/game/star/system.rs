use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::comp::*;
use super::consts::*;
use super::res::*;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(window) = window_query.get_single() {
        for _ in 0..NUM_OF_STARS {
            let x = random::<f32>() * window.width();
            let y = random::<f32>() * window.height();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},
            ));
        }
    }
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for entity in star_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn tick_star_spawn_timer(mut start_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    start_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    start_spawn_timer: Res<StarSpawnTimer>,
) {
    if start_spawn_timer.timer.finished() {
        if let Ok(window) = window_query.get_single() {
            let x = random::<f32>() * window.width();
            let y = random::<f32>() * window.height();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},
            ));
        }
    }
}
