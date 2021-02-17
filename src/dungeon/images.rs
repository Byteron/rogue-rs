use bevy::{prelude::*, utils::HashMap};

use super::{actor::ActorType, tile::TileType};

pub struct TileImages {
    images: HashMap<TileType, Handle<ColorMaterial>>,
}

impl TileImages {
    pub fn get(&self, image: TileType) -> Handle<ColorMaterial> {
        self.images.get(&image).unwrap().clone()
    }
}

impl FromResources for TileImages {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let assets = resources.get_mut::<AssetServer>().unwrap();

        let mut images: HashMap<TileType, Handle<ColorMaterial>> = HashMap::default();

        images.insert(
            TileType::Wall,
            materials.add(assets.load("images/wall.png").into()),
        );
        images.insert(
            TileType::Floor,
            materials.add(assets.load("images/floor.png").into()),
        );

        TileImages { images }
    }
}

pub struct ActorImages {
    images: HashMap<ActorType, Handle<ColorMaterial>>,
}

impl ActorImages {
    pub fn get(&self, image: ActorType) -> Handle<ColorMaterial> {
        self.images.get(&image).unwrap().clone()
    }
}

impl FromResources for ActorImages {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let assets = resources.get_mut::<AssetServer>().unwrap();

        let mut images: HashMap<ActorType, Handle<ColorMaterial>> = HashMap::default();

        images.insert(
            ActorType::Human,
            materials.add(assets.load("images/human.png").into()),
        );
        images.insert(
            ActorType::Goblin,
            materials.add(assets.load("images/goblin.png").into()),
        );

        ActorImages { images }
    }
}
