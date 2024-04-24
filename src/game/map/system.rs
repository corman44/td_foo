use bevy::{ecs::query, prelude::*, render::camera::{self, CameraProjection, RenderTarget}, text};
use bevy_ecs_ldtk::prelude::*;
use serde::de;

use crate::{main_menu::style::MARGIN, AppState, MyCameraMarker};
use super::components::AnimateRotation;

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

impl Map {
    // TODO: implement map spawning and despawning

}

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

// TODO: move camera to view the map
pub fn show_map(
    mut query_camera: Query<&mut OrthographicProjection, With<MyCameraMarker>>,
) {
    use bevy::render::camera::ScalingMode;

    let mut projection = query_camera.single_mut();
    projection.scaling_mode = ScalingMode::WindowSize(4.0);
    
}

// TODO: implement viewing of Map when transitioning to GameState
pub fn map_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // TODO: Spawn Text that spins with other Camera
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Animate Rotation", get_text_style(&asset_server)),
            ..default()
        },
        AnimateRotation,
    ));
}

pub fn animate_rotation(
    time: Res<Time>,
    mut text_query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in &mut text_query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos());
    }

}

pub fn get_text_style(
    asset_server: &Res<AssetServer>,
) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Kenney Future.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}