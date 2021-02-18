use crate::dungeon::DungeonPlugin;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

mod core;
mod dungeon;

pub const APPSTATE_UPDATE: &str = "AppStateUpdate";
pub const PHYSICS_UPDATE: &str = "PhysicsUpdate";
pub const APPSTATE_LATE_UPDATE: &str = "LateUpdate";
pub const VIEW_STARTUP: &str = "ViewSetup";
pub const VIEW_UPDATE: &str = "ViewStage";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Dungeon,
}

fn main() {
    App::build()
        // Debug
        // .insert_resource(ReportExecutionOrderAmbiguities)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .insert_resource(bevy::log::LogSettings {
        //     level: bevy::log::Level::DEBUG,
        //     ..Default::default()
        // })
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
            stage::UPDATE,
            APPSTATE_UPDATE,
            StateStage::<AppState>::default(),
        )
        .add_stage_after(
            APPSTATE_UPDATE,
            PHYSICS_UPDATE,
            StateStage::<AppState>::default(),
        )
        .add_stage_after(
            PHYSICS_UPDATE,
            APPSTATE_LATE_UPDATE,
            StateStage::<AppState>::default(),
        )
        .add_stage_after(
            APPSTATE_LATE_UPDATE,
            VIEW_STARTUP,
            StateStage::<AppState>::default(),
        )
        .add_stage_after(VIEW_STARTUP, VIEW_UPDATE, StateStage::<AppState>::default())
        // App State
        .insert_resource(State::new(AppState::Dungeon))
        // State Plugins
        .add_plugin(DungeonPlugin)
        .run();
}
