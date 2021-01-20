mod core;
mod dungeon;
mod tween;
mod player;
mod rogue;

use bevy::prelude::*;
use rogue::Rogue;

fn main() {
    App::build()
        .add_plugin(Rogue)
        .run();
}
