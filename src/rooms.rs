use std::hash::Hash;

use bevy::{app::startup_stage, prelude::*, render::camera::Camera, utils::HashMap};
use rand::Rng;

use crate::{
    enemies::{Enemies, EnemyImages, EnemyType},
    grid::{Grid, Vec2i, Vec3i},
    rogue::GameState,
};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum TileType {
    Floor,
    Wall,
}

pub struct TileImages(pub HashMap<TileType, Handle<ColorMaterial>>);

pub struct Tiles(pub HashMap<Vec3i, TileType>);

pub struct Rooms(pub HashMap<Vec3i, Room>);

pub struct Room {
    pub position: Vec2i,
    pub size: Vec2i,
}

impl Room {
    pub fn end(&self) -> Vec2i {
        self.position + self.size
    }

    pub fn last(&self) -> Vec2i {
        self.end() - Vec2i::new(1, 1)
    }

    pub fn center(&self) -> Vec2i {
        self.position + self.size / Vec2i::new(2, 2)
    }

    pub fn coords(&self) -> Vec<Vec2i> {
        let mut coords: Vec<Vec2i> = Vec::default();
        for y in self.position.y..self.end().y {
            for x in self.position.x..self.end().x {
                coords.push(Vec2i::new(x, y))
            }
        }

        coords
    }

    pub fn is_door(&self, coords: Vec2i) -> bool {
        coords.x == self.center().x && (coords.y == self.position.y || coords.y == self.last().y)
            || coords.y == self.center().y
                && (coords.x == self.position.x || coords.x == self.last().x)
    }

    pub fn is_exit(&self, coords: Vec2i) -> bool {
        coords.x == self.center().x && (coords.y == self.position.y - 1 || coords.y == self.end().y)
            || coords.y == self.center().y
                && (coords.x == self.position.x - 1 || coords.x == self.end().x)
    }

    pub fn is_border(&self, coords: Vec2i) -> bool {
        coords.x == self.position.x
            || coords.y == self.position.y
            || coords.x == self.last().x
            || coords.y == self.last().y
    }
}

pub fn create_room(tiles: &mut Tiles, level: i32, position: Vec2i, size: Vec2i) -> Room {
    let room = Room { position, size };

    let mut rng = rand::thread_rng();

    for coords in room.coords().iter_mut() {
        if room.is_door(*coords) {
            tiles.0.insert(coords.extend(level), TileType::Floor);
        } else if room.is_border(*coords) {
            tiles.0.insert(coords.extend(level), TileType::Wall);
        } else if rng.gen_bool(0.1) {
            tiles.0.insert(coords.extend(level), TileType::Wall);
        } else {
            tiles.0.insert(coords.extend(level), TileType::Floor);
        }
    }

    room
}

pub struct RoomsPlugin;

impl Plugin for RoomsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(TileImages(HashMap::default()))
            .add_resource(Rooms(HashMap::default()))
            .add_resource(Tiles(HashMap::default()))
            .add_startup_system_to_stage(startup_stage::PRE_STARTUP, setup.system());
    }
}

fn setup(
    assets: Res<AssetServer>,
    mut images: ResMut<TileImages>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    images.0.insert(
        TileType::Floor,
        materials.add(assets.load("images/floor.png").into()),
    );
    images.0.insert(
        TileType::Wall,
        materials.add(assets.load("images/wall.png").into()),
    );
}
