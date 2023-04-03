use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample8)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .run();
}

const PI: f32 = std::f32::consts::PI;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..Default::default()
    // });

    // Plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(500000.0).into()),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });
    //
    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 500.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Camera
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(10.0, 10.0, 10.0)
    //         .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    //     ..default()
    // });

    commands
        .spawn((
            Camera3dBundle::default(),
            FogSettings {
                color: Color::rgba(0.1, 0.2, 0.4, 1.0),
                directional_light_color: Color::rgba(1.0, 0.95, 0.75, 0.5),
                directional_light_exponent: 30.0,
                falloff: FogFalloff::from_visibility_colors(
                    100.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                    Color::rgb(0.1, 0.1, 0.1), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                    Color::rgb(0.9, 0.9, 0.9), // atmospheric inscattering color (light gained due to scattering from the sun)
                ),
            },
        ))
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));

    commands.spawn(SceneBundle {
        scene: asset_server.load("models/environment.glb#Scene0"),
        ..default()
    });
}
