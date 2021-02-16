use bevy::{prelude::*, utils::HashMap};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Image {
    Floor,
    Wall,
    Human,
    Goblin,
}

pub struct Images {
    images: HashMap<Image, Handle<ColorMaterial>>,
}

impl Images {
    pub fn get(&self, image: Image) -> Handle<ColorMaterial> {
        self.images.get(&image).unwrap().clone()
    }
}

impl FromResources for Images {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let assets = resources.get_mut::<AssetServer>().unwrap();

        let mut images: HashMap<Image, Handle<ColorMaterial>> = HashMap::default();

        images.insert(
            Image::Human,
            materials.add(assets.load("images/human.png").into()),
        );
        images.insert(
            Image::Goblin,
            materials.add(assets.load("images/goblin.png").into()),
        );
        images.insert(
            Image::Wall,
            materials.add(assets.load("images/wall.png").into()),
        );
        images.insert(
            Image::Floor,
            materials.add(assets.load("images/floor.png").into()),
        );

        Images { images }
    }
}
