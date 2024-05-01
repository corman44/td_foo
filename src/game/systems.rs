use bevy::prelude::*;

use crate::AppState;

use super::GameState;

pub fn pause_game(
    mut next_game_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>,
    app_state: Res<State<AppState>>,
    input: Res<Input<KeyCode>>,
) {
    if *app_state.get() == AppState::Game {
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
}