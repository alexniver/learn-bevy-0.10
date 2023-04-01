use bevy::prelude::*;

use super::InGameState;

pub fn toggle_in_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    in_game_state: Res<State<InGameState>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if in_game_state.0 == InGameState::Running {
            next_in_game_state.set(InGameState::Paused);
            println!("Paused");
        }
        if in_game_state.0 == InGameState::Paused {
            next_in_game_state.set(InGameState::Running);
            println!("Running");
        }
    }
}
