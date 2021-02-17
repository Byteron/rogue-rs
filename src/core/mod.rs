use bevy::prelude::*;

pub mod math;

pub const APPSTATES: &str = "AppStates";
pub const VIEW_STAGE: &str = "ViewStage";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Dungeon,
}

pub fn despawn_all<T: Component>(commands: &mut Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.despawn_recursive(e);
    }
}
