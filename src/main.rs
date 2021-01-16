mod core;
mod rogue;
mod player;

use bevy::diagnostic::PrintDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use rogue::Rogue;

fn main() {
    App::build()
    .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(Rogue)
        .run();
}
