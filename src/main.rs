mod rogue;
mod tile_map;
mod components;

use bevy::prelude::*;
use rogue::Rogue;


fn main() {
    App::build().add_plugin(Rogue).run();
}
