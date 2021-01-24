use bevy::{app::startup_stage, prelude::*, utils::HashMap};
use rand::Rng;

use crate::{rooms::{Rooms, TileType, Tiles}, grid::Vec3i};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum EnemyType {
    Goblin,
}

pub type EnemyImages = HashMap<EnemyType, Handle<ColorMaterial>>;

pub type Enemies = HashMap<Vec3i, EnemyType>;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(EnemyImages::default())
            .add_resource(Enemies::default())
            .add_startup_system_to_stage(startup_stage::PRE_STARTUP, pre_setup.system())
            .add_startup_system_to_stage(startup_stage::POST_STARTUP, setup.system());
    }
}

fn pre_setup(
    assets: Res<AssetServer>,
    mut images: ResMut<EnemyImages>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    images.insert(
        EnemyType::Goblin,
        materials.add(assets.load("images/goblin.png").into()),
    );
}

fn setup(
    tiles: Res<Tiles>,
    mut enemies: ResMut<Enemies>,
) {
    let mut rng = rand::thread_rng();

    for (coords, tile) in tiles.iter() {
        if *tile == TileType::Floor {
            if rng.gen_bool(0.01) {
                enemies.insert(*coords, EnemyType::Goblin);
            }
        }
    }
}
