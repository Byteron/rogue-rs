use std::hash::Hash;

use crate::{
    core::{self, Coordinates, Grid, Stepper},
    player,
    player::*,
};

use bevy::{prelude::*, utils::HashMap};

use rand::Rng;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Tile {
    Wall,
    Floor,
}

pub struct Room {
    pub tiles: HashMap<Coordinates, Tile>,
}

pub struct TileSet {
    pub tiles: HashMap<Tile, Handle<ColorMaterial>>,
}

impl TileSet {
    pub fn get(&self, tile: Tile) -> Handle<ColorMaterial> {
        self.tiles.get(&tile).unwrap().clone()
    }
}

pub struct Rogue;

impl Plugin for Rogue {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_resource(Grid::default())
            .add_startup_system(setup.system())
            .add_system(core::step.system())
            .add_system(player::input.system());
    }
}

fn setup(
    commands: &mut Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut tile_set = TileSet {
        tiles: HashMap::default(),
    };

    tile_set.tiles.insert(
        Tile::Wall,
        materials.add(assets.load("images/wall.png").into()),
    );
    tile_set.tiles.insert(
        Tile::Floor,
        materials.add(assets.load("images/floor.png").into()),
    );

    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            material: materials.add(assets.load("images/player.png").into()),
            transform: Transform::from_translation(Vec3::zero()),
            sprite: Sprite {
                size: Vec2::new(64.0, 64.0),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(Player)
        .with(Stepper::default())
        .with(Coordinates::zero());

    let mut room = generate_room();

    spawn_room(commands, &mut tile_set, &mut room);

    commands.insert_resource(room);
}

fn generate_room() -> Room {
    let mut room = Room {
        tiles: HashMap::default(),
    };

    let extents = Coordinates::new(9, 5);
    let mut rng = rand::thread_rng();

    for y in -extents.y..=extents.y {
        for x in -extents.x..=extents.x {
            if x == -extents.x || x == extents.x || y == -extents.y || y == extents.y {
                room.tiles.insert(Coordinates::new(x, y), Tile::Wall);
            } else if rng.gen_bool(0.3) {
                room.tiles.insert(Coordinates::new(x, y), Tile::Wall);
            } else {
                room.tiles.insert(Coordinates::new(x, y), Tile::Floor);
            }
        }
    }

    room
}

fn spawn_room(commands: &mut Commands, tile_set: &mut TileSet, room: &mut Room) {
    for (coords, tile) in room.tiles.iter() {
        commands
            .spawn(SpriteBundle {
                material: tile_set.get(*tile),
                transform: Transform::from_translation(
                    coords.to_vec().extend(-0.1) * Vec3::new(64.0, 64.0, 1.0),
                ),
                sprite: Sprite {
                    size: Vec2::new(64.0, 64.0),
                    resize_mode: SpriteResizeMode::Manual,
                },
                ..Default::default()
            })
            .with(*coords);
    }
}
