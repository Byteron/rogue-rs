use std::hash::Hash;

use bevy::{app::startup_stage, prelude::*, utils::HashMap};

use crate::shared::math::Vec3i;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum CharacterType {
    Human,
    Goblin,
}

pub struct CharacterImages(pub HashMap<CharacterType, Handle<ColorMaterial>>);

pub struct Characters(pub HashMap<Vec3i, Entity>);

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(CharacterImages(HashMap::default()))
            .insert_resource(Characters(HashMap::default()))
            .add_startup_system_to_stage(startup_stage::PRE_STARTUP, setup.system());
    }
}

fn setup(
    assets: Res<AssetServer>,
    mut images: ResMut<CharacterImages>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    images.0.insert(
        CharacterType::Goblin,
        materials.add(assets.load("images/goblin.png").into()),
    );

    images.0.insert(
        CharacterType::Human,
        materials.add(assets.load("images/human.png").into()),
    );
}
