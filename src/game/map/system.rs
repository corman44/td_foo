use bevy::{prelude::*, render::camera::{RenderTarget, ScalingMode}, utils::hashbrown::HashSet};
use bevy_ecs_ldtk::prelude::*;

use crate::MyCameraMarker;

use super::AttackerArea;

pub fn debug_cameras(
    q: Query<&Camera>,
) {
    for camera in &q {
        match &camera.target {
            RenderTarget::Window(wid) => {
                info!("Camera renders to window with id: {:?}", wid);
            }
            RenderTarget::Image(handle) => {
                info!("Camera renders to image asset with id: {:?}", handle);
            },
            RenderTarget::TextureView(_) => {
                info!("This is a special camera that outputs to something outside of Bevy.");
            },
        }
    }
}

pub fn zoom_to_map(
    _commands: Commands,
    mut camera_query: Query<(&mut OrthographicProjection, & mut Transform), With<MyCameraMarker>>,
) {
    let (mut camera, mut trans) = camera_query.single_mut();
    camera.viewport_origin = Vec2::ZERO;
    camera.scaling_mode = ScalingMode::Fixed { width: 1024.0, height: 1024.0 };
    trans.translation.x = 0.0;
    trans.translation.y = 0.0;
}

pub fn map_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let _map = commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk_map/td_foo_map1.ldtk").into(),
        ..Default::default()
    });
}

pub fn attack_locations(
    attack_tiles: Query<&GridCoords, With<AttackerArea>>
) {
    let _tiles: HashSet<GridCoords> = attack_tiles.iter().copied().collect();
    // info!("Attack Tiles: {:?}", tiles);

}

const GRID_SIZE: i32 = 64;

pub fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);
    }
}
