use crate::{
    core::{AppState, APPSTATES},
    dungeon::DungeonPlugin,
};
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

mod core;
mod dungeon;

fn main() {
    App::build()
        // Debug
        // .insert_resource(ReportExecutionOrderAmbiguities)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        // General
        .add_plugins(DefaultPlugins)
        // Window
        .insert_resource(WindowDescriptor {
            title: "ROGUE™ PRE-ALPHA".into(),
            vsync: false,
            resizable: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        // App State
        .add_stage_before(stage::UPDATE, APPSTATES, StateStage::<AppState>::default())
        .insert_resource(State::new(AppState::Dungeon))
        // State Plugins
        .add_plugin(DungeonPlugin)
        .run();
}
