mod core;
mod despawn;
mod dungeon;
mod images;
mod player;
mod rogue;
mod tween;

use bevy::prelude::*;
use rogue::Rogue;

fn main() {
    App::build().add_plugin(Rogue).run();
}
