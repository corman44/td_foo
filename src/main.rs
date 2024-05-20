use bevy::prelude::*;
use td_foo::AppPlugin;

// TODO: create "shooting" for Defender if Attacker is in range
// TODO: "place" defender in allowed spaces
// TODO: highlight defender RED if over a location not allowed
// FIXME: don't break if mouse moves off screen while NewDefender1 Selected

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run()
}
