use bevy::prelude::*;
use td_foo::AppPlugin;

// TODO: spawn Attackers with time delays
// TODO: create "start" button to start a round in game
// TODO: create "health" asset for defender
// TODO: decrement "health" if attacker reaches end of final tile

/* --- Developer Info ---
    * Controls
        S -> Spawn Tank at starting pos
 */

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run()
}