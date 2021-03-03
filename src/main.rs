use crate::dungeon::DungeonPlugin;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

mod core;
mod dungeon;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Dungeon,
}

fn main() {
    App::build()
        // Debug
        .insert_resource(ReportExecutionOrderAmbiguities)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
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
        // Stages
        .add_stage_before(
            CoreStage::Update,
            dungeon::Stage::Update,
            StateStage::<AppState>::default(),
        )
        .add_stage_after(
            dungeon::Stage::Update,
            dungeon::Stage::DamageUpdate,
            StateStage::<AppState>::default(),
        )
        .add_stage_after(
            dungeon::Stage::DamageUpdate,
            dungeon::Stage::PhysicsUpdate,
            StateStage::<AppState>::default(),
        )
        .add_stage_after(
            dungeon::Stage::PhysicsUpdate,
            dungeon::Stage::SyncUpdate,
            StateStage::<AppState>::default(),
        )
        // App State
        .insert_resource(State::new(AppState::Dungeon))
        // State Plugins
        .add_plugin(DungeonPlugin)
        .run();
}