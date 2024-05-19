use bevy::{prelude::*, window::PrimaryWindow};

use super::components::{Active, Defender, DefenderBundle};

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

pub fn move_selected_defender(
    mut defender_query: Query<(&Defender, &mut Transform), With<Active>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (defender, mut def_trans) in defender_query.iter_mut() {
        // if defender.spawning {
            let mut xy_pos = window_query.single().cursor_position().unwrap();
            def_trans.translation = Vec3::new(xy_pos.x, (xy_pos.y - 1024.).abs(), 6.0);
        // }
    }
}
