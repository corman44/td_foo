pub mod systems;
pub mod components;

use bevy::prelude::*;

use self::systems::{mouse_button_input, print_state_change};

pub struct DebuggingPlugin;

impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_state_change);
    }
}
