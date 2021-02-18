use bevy::prelude::*;

pub mod math;

pub fn despawn_all<T: Component>(commands: &mut Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.despawn_recursive(e);
    }
}
