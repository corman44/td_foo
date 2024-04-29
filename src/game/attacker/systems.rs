use bevy::prelude::*;

use super::components::Attacker;

pub struct AttackerPlugin;

impl Plugin for AttackerPlugin {
    fn build(&self, app: &mut App) {
    }
}

// TODO: move enemy in all directions from function
// TODO: define collisions with "Predefined Enemy Path"
//  - Goal is to have eney follow the "Attacker" path from LDTK

// TODO: load enemy asset at starting position
pub fn spawn_attacker(
    mut command: Commands,
    asset_server: Res<AssetServer>,
) {

}

pub fn move_attackers(
    mut attacker_query: Query<&mut Transform, With<Attacker>>,
) {
    for mut attacker in attacker_query.iter_mut() {
        if attacker.translation.y >= -550.0 {
            attacker.translation.y -= 1.0;
        }
    }
}