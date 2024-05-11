use bevy::prelude::*;

use crate::AppState;

use super::GameState;

pub fn pause_game(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    game_state: Res<State<GameState>>,
    app_state: Res<State<AppState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if *game_state.get() == GameState::Running {
            next_game_state.set(GameState::Paused);
        } else if *game_state.get() == GameState::Paused {
            next_game_state.set(GameState::Running);
        }
    }
}
