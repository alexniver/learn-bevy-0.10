use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use super::comp::*;
use super::consts::*;

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUM_OF_ENEMIES {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                dir: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_enemy_timer(mut timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

pub fn spawn_enemy_by_timer(
    mut commands: Commands,
    timer: Res<EnemySpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(window) = window_query.get_single() {
        if timer.timer.finished() {
            let x = random::<f32>() * window.width();
            let y = random::<f32>() * window.height();
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    dir: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            ));
        }
    }
}

pub fn enemy_move(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let dir = Vec3::new(enemy.dir.x, enemy.dir.y, 0.0);
        transform.translation += dir * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_dir(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // audio: Res<Audio>,
    // asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let x_min = 0.0 + HALF_ENEMY_SIZE;
    let x_max = window.width() - HALF_ENEMY_SIZE;
    let y_min = 0.0 + HALF_ENEMY_SIZE;
    let y_max = window.height() - HALF_ENEMY_SIZE;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        // let mut dir_changed = false;
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.dir.x *= -1.0;
            // dir_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.dir.y *= -1.0;
            // dir_changed = true;
        }

        // if dir_changed {
        //     let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
        //     let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
        //     let sound_effect = if random::<f32>() > 0.5 {
        //         sound_effect_1
        //     } else {
        //         sound_effect_2
        //     };
        //     audio.play(sound_effect);
        // }
    }
}
pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let x_min = 0.0 + HALF_ENEMY_SIZE;
    let x_max = window.width() - HALF_ENEMY_SIZE;
    let y_min = 0.0 + HALF_ENEMY_SIZE;
    let y_max = window.height() - HALF_ENEMY_SIZE;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;
        translation.x = translation.x.max(x_min);
        translation.x = translation.x.min(x_max);
        translation.y = translation.y.max(y_min);
        translation.y = translation.y.min(y_max);
        transform.translation = translation;
    }
}
