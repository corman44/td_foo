pub mod system;
pub mod components;

use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection};

use crate::AppState;

use self::system::{ debug_cameras, map_setup, zoom_to_map};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LdtkPlugin)
            .add_systems(Startup, map_setup)
            .insert_resource(LevelSelection::index(0))
            .add_systems(OnEnter(AppState::Game), (debug_cameras, zoom_to_map));
    }
}