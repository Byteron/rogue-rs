use bevy::{ecs::component::Component, prelude::*};

pub mod math;

pub fn despawn_all<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.despawn_recursive(e);
    }
}