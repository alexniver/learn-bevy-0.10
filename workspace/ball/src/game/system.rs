use bevy::prelude::*;

use super::InGameState;

pub fn toggle_in_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    in_game_state: Res<State<InGameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if in_game_state.0 == InGameState::Running {
            commands.insert_resource(NextState(Some(InGameState::Paused)));
            println!("Paused");
        }
        if in_game_state.0 == InGameState::Paused {
            commands.insert_resource(NextState(Some(InGameState::Running)));
            println!("Running");
        }
    }
}
