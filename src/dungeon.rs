use std::hash::Hash;

use crate::core::*;
use bevy::{prelude::*, utils::HashMap};
use rand::Rng;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum EnemyType {
    Goblin,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum ItemType {
    Weapon,
    Gold,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum BoardObject {
    Player,
    Enemy(EnemyType),
    Item(ItemType),
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

pub struct Room {
    pub position: Coordinates,
    pub size: Coordinates,
    pub tiles: HashMap<Coordinates, TileType>,
    pub objects: HashMap<Coordinates, Vec<BoardObject>>,
    pub entities: Vec<Entity>,
}

impl Room {
    pub fn end(&self) -> Coordinates {
        self.position + self.size
    }

    pub fn last(&self) -> Coordinates {
        self.end() - Coordinates::new(1, 1)
    }

    pub fn center(&self) -> Coordinates {
        self.position + self.size / Coordinates::new(2, 2)
    }

    pub fn random(&self) -> Coordinates {
        let mut tiles: Vec<Coordinates> = Vec::default();

        for (coords, tile) in self.tiles.iter() {
            if *tile == TileType::Floor {
                tiles.push(*coords);
            }
        }

        let mut rng = rand::thread_rng();

        *tiles.get(rng.gen_range(0..tiles.len())).unwrap()
    }

    pub fn coords(&self) -> Vec<Coordinates> {
        let mut coords: Vec<Coordinates> = Vec::default();
        for y in self.position.y..self.end().y {
            for x in self.position.x..self.end().x {
                coords.push(Coordinates::new(x, y))
            }
        }

        coords
    }

    pub fn is_door(&self, coords: Coordinates) -> bool {
        coords.x == self.center().x && (coords.y == self.position.y || coords.y == self.last().y)
            || coords.y == self.center().y
                && (coords.x == self.position.x || coords.x == self.last().x)
    }

    pub fn is_exit(&self, coords: Coordinates) -> bool {
        coords.x == self.center().x && (coords.y == self.position.y - 1 || coords.y == self.end().y)
            || coords.y == self.center().y
                && (coords.x == self.position.x - 1 || coords.x == self.end().x)
    }

    pub fn is_border(&self, coords: Coordinates) -> bool {
        coords.x == self.position.x
            || coords.y == self.position.y
            || coords.x == self.last().x
            || coords.y == self.last().y
    }
}

pub struct Level {
    pub camera: Entity,
    rooms: HashMap<Coordinates, Room>,
    current: Coordinates,
}

impl Level {
    pub fn get_current_room(&mut self) -> &mut Room {
        self.rooms.get_mut(&self.current).unwrap()
    }

    pub fn change_current(&mut self, direction: Coordinates) {
        let new_current = self.current + direction;
        if self.rooms.contains_key(&new_current) {
            self.current = new_current;
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Level {
            camera: Entity::new(0),
            rooms: HashMap::default(),
            current: Coordinates::zero(),
        }
    }
}

pub fn generate() -> Level {
    let mut level = Level::default();

    let room_size = Coordinates::new(19, 11);
    let level_size = Coordinates::new(10, 10);

    for y in -level_size.y..=level_size.y {
        for x in -level_size.x..=level_size.x {
            let room_coords = Coordinates::new(x, y);

            let room = generate_room(
                room_coords * (room_size - Coordinates::new(1, 1)),
                room_size,
            );

            level.rooms.insert(room_coords, room);
        }
    }

    level
}

pub fn generate_room(position: Coordinates, size: Coordinates) -> Room {
    let mut room = Room {
        position,
        size,
        tiles: HashMap::default(),
        objects: HashMap::default(),
        entities: Vec::default(),
    };

    let mut rng = rand::thread_rng();

    for coords in room.coords().iter() {
        if room.is_door(*coords) {
            room.tiles.insert(*coords, TileType::Floor);
        } else if room.is_border(*coords) {
            room.tiles.insert(*coords, TileType::Wall);
        } else if rng.gen_bool(0.1) {
            room.tiles.insert(*coords, TileType::Wall);
        } else {
            room.tiles.insert(*coords, TileType::Floor);
        }
    }

    for (coords, tile) in room.tiles.iter() {
        if *tile == TileType::Floor {
            if rng.gen_bool(0.01) {
                match room.objects.get_mut(coords) {
                    Some(objects) => objects.push(BoardObject::Enemy(EnemyType::Goblin)),
                    None => {
                        let mut vec: Vec<BoardObject> = Vec::default();
                        vec.push(BoardObject::Enemy(EnemyType::Goblin));
                        room.objects.insert(*coords, vec);
                    }
                }
            }
        }
    }

    room
}

pub fn spawn_room(commands: &mut Commands, grid: &Grid, images: &Images, room: &mut Room) {
    let mut entities: Vec<Entity> = Vec::default();

    for (coords, tile) in room.tiles.iter() {
        let entity = spawn_tile(commands, grid, images, *tile, *coords);
        entities.push(entity);
    }

    for (coords, objects) in room.objects.iter() {
        for bob in objects.iter() {
            match bob {
                BoardObject::Player => {}
                BoardObject::Enemy(enemy) => {
                    let entity = spawn_enemy(commands, grid, images, *enemy, *coords);
                    entities.push(entity);
                }
                BoardObject::Item(item) => {}
            }
        }
    }

    for entity in entities.iter() {
        room.entities.push(*entity);
    }
}

pub fn despawn_room(commands: &mut Commands, room: &mut Room) {
    for entity in room.entities.iter() {
        commands.despawn(*entity);
    }

    room.entities.clear();
}

fn spawn_tile(
    commands: &mut Commands,
    grid: &Grid,
    images: &Images,
    tile: TileType,
    coords: Coordinates,
) -> Entity {
    let translation = grid.map_to_world(coords) + Vec3::new(0.0, 0.0, -0.1);

    commands
        .spawn(SpriteBundle {
            material: images.get_tile(tile),
            transform: Transform::from_translation(translation),
            sprite: Sprite {
                size: grid.cell_size,
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(coords)
        .current_entity()
        .unwrap()
}

fn spawn_enemy(
    commands: &mut Commands,
    grid: &Grid,
    images: &Images,
    enemy: EnemyType,
    coords: Coordinates,
) -> Entity {
    let translation = grid.map_to_world(coords);

    commands
        .spawn(SpriteBundle {
            material: images.get_enemy(enemy),
            transform: Transform::from_translation(translation),
            sprite: Sprite {
                size: grid.cell_size,
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(EnemyType::Goblin)
        .with(Stepper::new(translation))
        .with(coords)
        .current_entity()
        .unwrap()
}
