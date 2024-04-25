use bevy::{prelude::*, render::camera::{RenderTarget, ScalingMode}, transform};
use bevy_ecs_ldtk::prelude::*;

use crate::MyCameraMarker;

pub fn debug_cameras(
    q: Query<&Camera>,
) {
    for camera in &q {
        match &camera.target {
            RenderTarget::Window(wid) => {
                eprintln!("Camera redners to window with id: {:?}", wid);
            }
            RenderTarget::Image(handle) => {
                eprintln!("Camera renders to image asset with id: {:?}", handle);
            },
            RenderTarget::TextureView(_) => {
                eprintln!("This is a special camera that outputs to something outside of Bevy.");
            },
        }
    }
}

pub fn zoom_to_map(
    mut commands: Commands,
    mut camera_query: Query<(&mut OrthographicProjection, & mut Transform), With<MyCameraMarker>>,
) {
    let (mut camera, mut trans) = camera_query.single_mut();
    camera.viewport_origin = Vec2::ZERO;
    camera.scaling_mode = ScalingMode::Fixed { width: 1024.0, height: 1024.0 };
    // TODO: maybe add translation to camera Query for moving camera to 512, 512?
    trans.translation.x = 0.0;
    trans.translation.y = 0.0;
}

pub fn map_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let map = commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk_map/td_foo_map1.ldtk").into(),
        ..Default::default()
    });
}