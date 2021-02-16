use bevy::prelude::*;

pub mod images;
pub mod math;

pub const APPSTATES: &str = "AppStates";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Dungeon,
}

pub fn despawn_all<T: Component>(commands: &mut Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.despawn_recursive(e);
    }
}
