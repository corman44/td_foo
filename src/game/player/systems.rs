use bevy::prelude::*;

use super::PlayerHealth;

pub fn print_player_health(
    player_health: Res<PlayerHealth>,
) {
    info!("PlayerHealth: {}", player_health.0);
}