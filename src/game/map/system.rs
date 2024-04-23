use bevy::{prelude::*, render::camera::{self, CameraProjection, RenderTarget}};
use bevy_ecs_ldtk::prelude::*;

use crate::{AppState, MyCameraMarker};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Tile {
    Path,
    Tower,
    AttackStart,
    AttackEnd,
    Empty
}

#[derive(Debug, Resource)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub height: usize,
    pub width: usize,
}

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
            .add_systems(Startup, map_setup)
            .add_systems(OnEnter(AppState::Game), (show_map, debug_render_text));
    }
}

impl Map {
    // TODO: implement map creation/updates
}

pub fn debug_render_text(
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

// TODO: move camera to view the map
pub fn show_map(
    mut query_camera: Query<&mut OrthographicProjection, With<MyCameraMarker>>,
) {
    use bevy::render::camera::ScalingMode;

    let mut projection = query_camera.single_mut();
    projection.scaling_mode = ScalingMode::WindowSize(4.0);
    
}

// TODO: implement viewing of Map when transitioning to GameState
pub fn map_setup() {

}
