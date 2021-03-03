use bevy::{prelude::*, utils::HashMap};

pub struct Images {
    map: HashMap<&'static str, Handle<ColorMaterial>>,
}

impl Images {
    pub fn get(&self, name: &str) -> Handle<ColorMaterial> {
        self.map.get(name).unwrap().clone()
    }
}

impl FromResources for Images {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let assets = resources.get::<AssetServer>().unwrap();

        let mut map: HashMap<&'static str, Handle<ColorMaterial>> = HashMap::default();

        map.insert("Wall", materials.add(assets.load("images/wall.png").into()));
        map.insert(
            "Floor",
            materials.add(assets.load("images/floor.png").into()),
        );

        map.insert(
            "Human",
            materials.add(assets.load("images/human.png").into()),
        );
        map.insert(
            "Goblin",
            materials.add(assets.load("images/goblin.png").into()),
        );

        map.insert(
            "Sword",
            materials.add(assets.load("images/sword.png").into()),
        );

        Images { map }
    }
}
