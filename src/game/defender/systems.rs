use bevy::prelude::*;

use super::DefenderBundle;


pub fn spawn_blue_defender(
    assset_server: &Res<AssetServer>,
    mut commands: Commands,
) {
    let texture = assset_server.load(r"kenney_top-down-tanks-redux\PNG\Default size\tank_blue_cj.png");
    commands.spawn(
        DefenderBundle {
            sprite_bundle: SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(512., 512., 6.),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
    );
}
