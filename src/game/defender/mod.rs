pub mod systems;
pub mod components;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct DefenderPlugin;

impl Plugin for DefenderPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}