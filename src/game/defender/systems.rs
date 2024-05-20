use bevy::{prelude::*, window::PrimaryWindow};

use super::components::{Active, Defender, DefenderBundle};

pub fn spawn_blue_defender(
    assset_server: &Res<AssetServer>,
    mut commands: Commands,
) {
    let texture = assset_server.load(r"kenney_top-down-tanks-redux\PNG\Default size\tank_blue_cj.png");
    commands.spawn(
        DefenderBundle {
            active: Active(true),
            defender: Defender {
                attacking: false,
                attack_speed: 1,
                damage: 5,
                spawning: true,
            },
            sprite_bundle: SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(512., 512., 6.),
                    scale: Vec3::new(0.80, 0.80, 1.0), // scaling to not overlap with other defenders
                    ..default()
                },
                ..default()
            },
            ..default()
        },
    );
}

pub fn move_selected_defender(
    mut defender_query: Query<(&Defender, &mut Transform, &Active)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (defender, mut def_trans, active) in defender_query.iter_mut() {
        if defender.spawning && active.0 {
            let xy_pos = window_query.single().cursor_position().unwrap();
            def_trans.translation = Vec3::new(xy_pos.x, (xy_pos.y - 1024.).abs(), 6.0);
        }
    }
}

pub fn selected_defender_moving_highlight(
    
) {
    // TODO: check if current location is in defender area
    // TODO: check if current location is blocked by another defender
}

pub fn place_selected_defender(
    mut defender_query: Query<(&mut Defender, &Active)>
) {
    for (mut defender, active) in defender_query.iter_mut() {
        if defender.spawning && active.0 {
            defender.spawning = false;
        }
    }
}
