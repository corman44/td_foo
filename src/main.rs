use bevy::prelude::*;
use td_foo::AppPlugin;

// TODO: spawn Attackers with time delays
// TODO: fix broken Game/App State and pause menu spawning...
/* --- Developer Info ---

    * Controls
        S -> Spawn Tank at starting pos
 */

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run()
}