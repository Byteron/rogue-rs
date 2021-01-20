use bevy::prelude::*;

pub struct Despawn;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(stage::POST_UPDATE, despawn.system());
    }
}

fn despawn(commands: &mut Commands, mut query: Query<Entity, Added<Despawn>>) {
    for entity in query.iter_mut() {
        commands.despawn(entity);
    }
}
