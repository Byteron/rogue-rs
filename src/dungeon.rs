use std::hash::Hash;

use crate::{
    core::*,
    images::Images,
    tween::Tween,
};
use bevy::{prelude::*, render::camera::Camera, utils::HashMap};
use rand::Rng;

struct Despawn;

pub struct ExitRoomEvent {
    pub direction: Coordinates,
}

struct EnterRoomEvent;

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

pub struct Room {
    pub position: Coordinates,
    pub size: Coordinates,
    pub tiles: HashMap<Coordinates, TileType>,
    pub objects: HashMap<Coordinates, Vec<BoardObject>>,
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

type Level = HashMap<Coordinates, Room>;

pub struct GameState {
    current_level: usize,
    current_room: Coordinates,
    levels: Vec<Level>,
}

impl GameState {
    pub fn add_level(&mut self) {
        self.levels.push(Level::default());
    }

    pub fn get_current_level(&mut self) -> &mut Level {
        self.levels.get_mut(self.current_level).unwrap()
    }

    pub fn get_current_room(&mut self) -> &mut Room {
        let current_room = self.current_room;
        self.get_current_level().get_mut(&current_room).unwrap()
    }

    pub fn change_current_level(&mut self, direction: usize) {
        let new_level = self.current_level + direction;
        if self.levels.len() > new_level {
            self.current_level = new_level;
        }
    }

    pub fn change_current_room(&mut self, direction: Coordinates) {
        let new_room = self.current_room + direction;
        if self.get_current_level().contains_key(&new_room) {
            self.current_room = new_room;
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            current_level: 0,
            current_room: Coordinates::zero(),
            levels: Vec::default(),
        }
    }
}

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ExitRoomEvent>()
            .add_event::<EnterRoomEvent>()
            .add_system(on_exit_room.system())
            .add_system(on_enter_room.system());
    }
}

fn on_exit_room(
    commands: &mut Commands,
    mut events: ResMut<Events<EnterRoomEvent>>,
    mut event_reader: EventReader<ExitRoomEvent>,
    mut state: ResMut<GameState>,
    mut active_entities: Query<Entity, With<Despawn>>,
) {
    for event in event_reader.iter() {
        despawn(commands, &mut active_entities);
        state.change_current_room(event.direction);
        events.send(EnterRoomEvent);
    }
}

fn on_enter_room(
    commands: &mut Commands,
    mut events: EventReader<EnterRoomEvent>,
    images: Res<Images>,
    grid: Res<Grid>,
    mut state: ResMut<GameState>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    for _ in events.iter() {
        let mut camera_transform = cameras.iter_mut().next().unwrap();
        let room = state.get_current_room();
        spawn_room(commands, &grid, &images, room);
        camera_transform.translation = grid.map_to_world(room.center());
    }
}

pub fn generate() -> GameState {
    let mut state = GameState::default();

    for _ in 0..=50 {
        state.add_level();
        state.change_current_level(1);

        let room_size = Coordinates::new(19, 11);
        let level_size = Coordinates::new(10, 10);

        for y in -level_size.y..=level_size.y {
            for x in -level_size.x..=level_size.x {
                let room_coords = Coordinates::new(x, y);

                let room = generate_room(
                    room_coords * (room_size - Coordinates::new(1, 1)),
                    room_size,
                );

                state.get_current_level().insert(room_coords, room);
            }
        }
    }

    state
}

fn generate_room(position: Coordinates, size: Coordinates) -> Room {
    let mut room = Room {
        position,
        size,
        tiles: HashMap::default(),
        objects: HashMap::default(),
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

fn spawn_room(commands: &mut Commands, grid: &Grid, images: &Images, room: &mut Room) {
    for (coords, tile) in room.tiles.iter() {
        spawn_tile(commands, grid, images, *tile, *coords);
    }

    for (coords, objects) in room.objects.iter() {
        for bob in objects.iter() {
            match bob {
                BoardObject::Player => {}
                BoardObject::Enemy(enemy) => {
                    spawn_enemy(commands, grid, images, *enemy, *coords);
                }
                BoardObject::Item(item) => {}
            }
        }
    }
}

fn spawn_tile(
    commands: &mut Commands,
    grid: &Grid,
    images: &Images,
    tile: TileType,
    coords: Coordinates,
) {
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
        .with(Despawn)
        .with(tile)
        .with(coords);
}

fn spawn_enemy(
    commands: &mut Commands,
    grid: &Grid,
    images: &Images,
    enemy: EnemyType,
    coords: Coordinates,
) {
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
        .with(Despawn)
        .with(enemy)
        .with(coords)
        .with(Tween::new(translation));
}

fn despawn(commands: &mut Commands, active_entities: &mut Query<Entity, With<Despawn>>) {
    for entity in active_entities.iter() {
        commands.despawn(entity);
    }
}
