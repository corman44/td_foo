pub mod systems;
pub mod components;

use bevy::prelude::*;

use self::systems::mouse_button_input;

pub struct DebuggingPlugin;

impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_button_input);
    }
}
