use crate::dungeon::*;
use bevy::{app::startup_stage, prelude::*, utils::HashMap};

pub struct ImagesPlugin;

impl Plugin for ImagesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(startup_stage::PRE_STARTUP, setup.system());
    }
}

pub struct Images {
    pub player: Handle<ColorMaterial>,
    pub tiles: HashMap<TileType, Handle<ColorMaterial>>,
    pub enemies: HashMap<EnemyType, Handle<ColorMaterial>>,
    pub items: HashMap<ItemType, Handle<ColorMaterial>>,
}

impl Images {
    pub fn add_player(
        &mut self,
        assets: &AssetServer,
        materials: &mut Assets<ColorMaterial>,
        path: &str,
    ) {
        self.player = materials.add(assets.load(path).into());
    }

    pub fn add_tile(
        &mut self,
        assets: &AssetServer,
        materials: &mut Assets<ColorMaterial>,
        tile: TileType,
        path: &str,
    ) {
        self.tiles
            .insert(tile, materials.add(assets.load(path).into()));
    }

    pub fn add_enemy(
        &mut self,
        assets: &AssetServer,
        materials: &mut Assets<ColorMaterial>,
        enemy: EnemyType,
        path: &str,
    ) {
        self.enemies
            .insert(enemy, materials.add(assets.load(path).into()));
    }

    pub fn add_item(
        &mut self,
        assets: &AssetServer,
        materials: &mut Assets<ColorMaterial>,
        item: ItemType,
        path: &str,
    ) {
        self.items
            .insert(item, materials.add(assets.load(path).into()));
    }

    pub fn get_player(&self) -> Handle<ColorMaterial> {
        self.player.clone()
    }

    pub fn get_tile(&self, tile: TileType) -> Handle<ColorMaterial> {
        self.tiles.get(&tile).unwrap().clone()
    }

    pub fn get_enemy(&self, enemy: EnemyType) -> Handle<ColorMaterial> {
        self.enemies.get(&enemy).unwrap().clone()
    }

    pub fn get_item(&self, item: ItemType) -> Handle<ColorMaterial> {
        self.items.get(&item).unwrap().clone()
    }
}

impl Default for Images {
    fn default() -> Self {
        Images {
            player: Handle::default(),
            tiles: HashMap::default(),
            enemies: HashMap::default(),
            items: HashMap::default(),
        }
    }
}

fn setup(
    commands: &mut Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut images = Images::default();

    images.add_player(&assets, &mut materials, "images/player.png");
    images.add_tile(&assets, &mut materials, TileType::Wall, "images/wall.png");
    images.add_tile(&assets, &mut materials, TileType::Floor, "images/floor.png");
    images.add_enemy(
        &assets,
        &mut materials,
        EnemyType::Goblin,
        "images/enemy.png",
    );

    commands.insert_resource(images);
}
