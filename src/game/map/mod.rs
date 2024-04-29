pub mod system;
pub mod components;

use bevy::prelude::*;
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, LdtkIntCell, LdtkPlugin, LevelSelection};

use crate::AppState;

use self::system::{ debug_cameras, map_setup, zoom_to_map};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct GridCoords {
    pub x: i32,
    pub y: i32,
}

impl From<IVec2> for GridCoords {
    fn from(i_vec_2: IVec2) -> Self {
        GridCoords {
            x: i_vec_2.x,
            y: i_vec_2.y,
        }
    }
}

impl From<GridCoords> for IVec2 {
    fn from(grid_coords: GridCoords) -> Self {
        IVec2::new(grid_coords.x, grid_coords.y)
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LdtkPlugin)
            .add_systems(Startup, map_setup)
            .insert_resource(LevelSelection::index(0))
            .add_systems(OnEnter(AppState::Game), (debug_cameras, zoom_to_map))
            .register_ldtk_int_cell::<DefenderAreaBundle>(1)
            .register_ldtk_int_cell::<AttackerAreaBundle>(2)
            .register_ldtk_int_cell::<UnusedAreaBundle>(3);
        }
}

#[derive(Default, Component)]
pub struct DefenderArea;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct DefenderAreaBundle {
    DefenderArea: DefenderArea,
}

#[derive(Default, Component)]
pub struct AttackerArea;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct AttackerAreaBundle {
    AttackerArea: AttackerArea,
}

#[derive(Default, Component)]
pub struct UnusedArea;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct UnusedAreaBundle {
    UnusedArea: UnusedArea,
}
