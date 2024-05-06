use bevy::prelude::*;

use self::system::track_click_and_drag;

pub mod system;
pub mod components;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, track_click_and_drag);
    }
}