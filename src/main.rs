mod components;
mod core;
mod rogue;
mod tile_map;

use bevy::prelude::*;
use rogue::Rogue;

fn main() {
    App::build().add_plugin(Rogue).run();
}
