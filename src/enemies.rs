use std::hash::Hash;

use bevy::{app::startup_stage, prelude::*, utils::HashMap};

use crate::grid::Vec3i;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum EnemyType {
    Goblin,
}

pub struct View(pub Option<Entity>);

pub struct EnemyImages(pub HashMap<EnemyType, Handle<ColorMaterial>>);

pub struct Enemies(pub HashMap<Vec3i, Entity>);

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(EnemyImages(HashMap::default()))
            .add_resource(Enemies(HashMap::default()))
            .add_startup_system_to_stage(startup_stage::PRE_STARTUP, pre_setup.system());
    }
}

fn pre_setup(
    assets: Res<AssetServer>,
    mut images: ResMut<EnemyImages>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    images.0.insert(
        EnemyType::Goblin,
        materials.add(assets.load("images/goblin.png").into()),
    );
}
