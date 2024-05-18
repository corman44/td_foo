use bevy::prelude::*;

use crate::{game::{attacker::AttackerSpawnState, defender::DefenderStats, GameState}, AppState};

use super::{DebugCounter, DebugTimer};

pub fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        info!("MouseButton::Left Pressed");
    }
    if buttons.just_released(MouseButton::Left) {
        info!("MouseButton::Left Release");
    }
    if buttons.pressed(MouseButton::Right) {
        info!("MouseButton::Right Pressed");
    }
    // we can check multiple at once with `.any_*`
    // if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
    //     // Either the left or the right button was just pressed
    // }
}

pub fn print_state_change(
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    attack_spawn_state: Res<State<AttackerSpawnState>>,
) {
    if app_state.is_changed() {
        info!("AppState::{:?}",app_state.get());
    }

    if game_state.is_changed() {
        info!("GameState::{:?}", game_state.get());
    }

    if attack_spawn_state.is_changed() {
        info!("AttackerSpawnState::{:?}", attack_spawn_state.get());
    }

}

pub fn print_defender_stats(
    defender_stats: Res<DefenderStats>,
) {
    info!("DefenderStats: {:?}", defender_stats);
}

pub fn debug_timer(
    mut debug_counter: ResMut<DebugCounter>,
    mut debug_timer: ResMut<DebugTimer>,
    time: Res<Time>,
) {
    if debug_timer.0.tick(time.delta()).just_finished() {
        debug_counter.0 += 1;
        // info!("DebugCounter: {}", debug_counter.0);
    };
}
