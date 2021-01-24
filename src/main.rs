mod dungeon;
mod rooms;
mod enemies;
mod grid;
mod player;
mod rogue;
mod tween;

use bevy::prelude::*;
use rogue::Rogue;

fn main() {
    App::build().add_plugin(Rogue).run();
}
