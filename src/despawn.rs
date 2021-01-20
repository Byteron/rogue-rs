use bevy::prelude::*;

use crate::core::Active;

pub struct Despawn;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(stage::POST_UPDATE, despawn.system());
    }
}

pub fn prepare_despawn(commands: &mut Commands, active_entities: &mut Query<Entity, With<Active>>) {
    for entity in active_entities.iter() {
        commands.insert_one(entity, Despawn);
    }
}

fn despawn(commands: &mut Commands, mut query: Query<Entity, Added<Despawn>>) {
    for entity in query.iter_mut() {
        commands.despawn(entity);
    }
}
