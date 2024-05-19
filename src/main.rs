use bevy::prelude::*;
use td_foo::AppPlugin;

// TODO: spawn Defender on Mouse Click
// TODO: create "shooting" for Defender if Attacker is in range

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run()
}
