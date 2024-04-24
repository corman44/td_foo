pub mod system;
pub mod components;

use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkPlugin;

use crate::AppState;

use self::system::{animate_rotation, debug_cameras, map_setup, show_map, Map};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let map = Map {
            tiles: vec![],
            height: 20,
            width: 20,
        };
        app
            .add_plugins(LdtkPlugin)
            .insert_resource(map)
            .add_systems(OnEnter(AppState::Game), (debug_cameras, map_setup))
            .add_systems(Update, animate_rotation);
    }
}