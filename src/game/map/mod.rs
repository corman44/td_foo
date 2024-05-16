pub mod system;
pub mod components;

use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_ldtk::prelude::*;

use crate::AppState;


use self::system::{debug_cameras, map_setup};


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LdtkPlugin)
            .add_systems(Startup, map_setup)
            .insert_resource(LevelSelection::index(0))
            .add_systems(OnEnter(AppState::Game), debug_cameras)
            .register_ldtk_int_cell::<DefenderAreaBundle>(1)
            .register_ldtk_int_cell::<AttackerAreaBundle>(2)
            .register_ldtk_int_cell::<UnusedAreaBundle>(3);
        }
}


#[derive(Default, Component)]
pub struct DefenderArea;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct DefenderAreaBundle {
    defender_area: DefenderArea,
}

#[derive(Default, Component)]
pub struct AttackerArea;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct AttackerAreaBundle {
    attack_area: AttackerArea,
}

#[derive(Default, Component)]
pub struct UnusedArea;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct UnusedAreaBundle {
    unused_area: UnusedArea,
}

#[derive(Default, Resource)]
pub struct AttackTiles {
    locations: HashSet<GridCoords>,
}

#[derive(Default, Resource)]
pub struct DefendTiles {
    locations: HashSet<GridCoords>,
}

#[derive(Default, Resource)]
pub struct UnusedTiles {
    locations: HashSet<GridCoords>,
}

#[derive(Default, Component)]
struct Tile;

#[derive(Default, Bundle, LdtkIntCell)]
struct TileBundle {
    tile: Tile,
}
