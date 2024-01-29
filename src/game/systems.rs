use bevy::prelude::*;

use super::GameState;

pub fn pause_game(
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_game_state.set(GameState::Paused);
}