use bevy::prelude::*;

use crate::main_menu::{comp::*, style::*};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(300.0), Val::Px(120.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                            margin: UiRect::new(
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                            ),
                            ..default()
                        },
                        image: asset_server.load("sprites/ball_blue_large.png").into(),
                        ..default()
                    });

                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Bevy Ball Game",
                                TextStyle {
                                    font: asset_server.load("font/FiraSans-Bold.ttf"),
                                    font_size: 64.0,
                                    color: Color::WHITE,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                            margin: UiRect::new(
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                            ),
                            ..default()
                        },
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });
                });
            // === Play Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                TextStyle {
                                    font: asset_server.load("font/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::rgb(1.0, 1.0, 1.0),
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Quit Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                TextStyle {
                                    font: asset_server.load("font/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::rgb(1.0, 1.0, 1.0),
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id()
}
