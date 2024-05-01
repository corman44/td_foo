use bevy::prelude::*;

use super::GameState;

pub fn pause_game(
    mut next_game_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::P) {
        let gs = game_state.get();
        if *gs == GameState::Paused {
            next_game_state.set(GameState::Running);
        }
        else if *gs == GameState::Running {
            next_game_state.set(GameState::Paused);
        }
    }
}