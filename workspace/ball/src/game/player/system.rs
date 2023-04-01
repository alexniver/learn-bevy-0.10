use bevy::{prelude::*, window::PrimaryWindow};

use super::comp::*;
use super::consts::*;
use crate::event::*;
use crate::game::enemy::comp::*;
use crate::game::enemy::consts::*;
use crate::game::score::comp::*;
use crate::game::star::comp::*;
use crate::game::star::consts::*;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() as f32 / 2.0,
                window.height() as f32 / 2.0,
                0.0,
            ),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::A) {
            dir += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            dir += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            dir += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            dir += Vec3::new(0.0, -1.0, 0.0);
        }

        if dir.length() > 0.0 {
            dir = dir.normalize();
        }

        transform.translation += dir * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = 0.0 + HALF_PLAYER_SIZE;
        let x_max = window.width() - HALF_PLAYER_SIZE;
        let y_min = 0.0 + HALF_PLAYER_SIZE;
        let y_max = window.height() - HALF_PLAYER_SIZE;

        let mut translation = player_transform.translation;
        translation.x = translation.x.max(x_min);
        translation.x = translation.x.min(x_max);
        translation.y = translation.y.max(y_min);
        translation.y = translation.y.min(y_max);
        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    // audio: Res<Audio>,
    // asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let radius_sum = HALF_ENEMY_SIZE + HALF_PLAYER_SIZE;
            if distance < radius_sum {
                println!("hit");
                // let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                // audio.play(sound_effect);
                commands.entity(player_entity).despawn();

                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}
pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    // audio: Res<Audio>,
    // asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < HALF_PLAYER_SIZE + HALF_STAR_SIZE {
                println!("hit star");
                // let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                // audio.play(sound_effect);
                commands.entity(star_entity).despawn();
                score.value += 1;
            }
        }
    }
}
