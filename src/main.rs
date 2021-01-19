mod core;
mod dungeon;
mod player;
mod rogue;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::PrintDiagnosticsPlugin;
use bevy::prelude::*;
use rogue::Rogue;

fn main() {
    App::build()
        // .add_plugin(FrameTimeDiagnosticsPlugin)
        // .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(Rogue)
        .run();
}
