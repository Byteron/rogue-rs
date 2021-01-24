use std::hash::Hash;

use bevy::{app::startup_stage, prelude::*, render::camera::Camera, utils::HashMap};
use rand::Rng;

use crate::{
    grid::{Grid, Vec2i, Vec3i},
    rogue::GameState,
};

pub struct Despawn;

pub struct RoomExitedEvent {
    pub direction: Vec2i,
}

pub struct RoomEnteredEvent;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum TileType {
    Floor,
    Wall,
}

pub type TileImages = HashMap<TileType, Handle<ColorMaterial>>;

pub type Tiles = HashMap<Vec3i, TileType>;

pub type Rooms = HashMap<Vec3i, Room>;

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

pub struct RoomsPlugin;

impl Plugin for RoomsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(TileImages::default())
            .add_resource(Rooms::default())
            .add_resource(Tiles::default())
            .add_event::<RoomExitedEvent>()
            .add_event::<RoomEnteredEvent>()
            .add_startup_system_to_stage(startup_stage::PRE_STARTUP, setup.system())
            .add_system(on_room_exited.system())
            .add_system(on_room_entered.system());
    }
}

fn setup(
    assets: Res<AssetServer>,
    mut images: ResMut<TileImages>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    images.insert(
        TileType::Floor,
        materials.add(assets.load("images/floor.png").into()),
    );
    images.insert(
        TileType::Wall,
        materials.add(assets.load("images/wall.png").into()),
    );
}

fn on_room_exited(
    commands: &mut Commands,
    rooms: Res<Rooms>,
    mut events: ResMut<Events<RoomEnteredEvent>>,
    mut event_reader: EventReader<RoomExitedEvent>,
    mut state: ResMut<GameState>,
    mut active_entities: Query<Entity, With<Despawn>>,
) {
    for event in event_reader.iter() {
        let new_room = (state.current_room.reduce() + event.direction).extend(state.current_level);

        if let Some(_) = rooms.get(&new_room) {
            despawn(commands, &mut active_entities);
            state.current_room = new_room;
            events.send(RoomEnteredEvent);
        }
    }
}

fn on_room_entered(
    commands: &mut Commands,
    images: Res<TileImages>,
    grid: Res<Grid>,
    rooms: Res<Rooms>,
    tiles: Res<Tiles>,
    state: Res<GameState>,
    mut events: EventReader<RoomEnteredEvent>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    for _ in events.iter() {
        let mut camera_transform = cameras.iter_mut().next().unwrap();
        let room = rooms.get(&state.current_room).unwrap();

        spawn_room(commands, &grid, &images, &tiles, state.current_level, &room);
        camera_transform.translation = grid.map_to_world(room.center());
    }
}

pub fn create_room(tiles: &mut Tiles, level: i32, position: Vec2i, size: Vec2i) -> Room {
    let room = Room { position, size };

    let mut rng = rand::thread_rng();

    for coords in room.coords().iter_mut() {
        if room.is_door(*coords) {
            tiles.insert(coords.extend(level), TileType::Floor);
        } else if room.is_border(*coords) {
            tiles.insert(coords.extend(level), TileType::Wall);
        } else if rng.gen_bool(0.1) {
            tiles.insert(coords.extend(level), TileType::Wall);
        } else {
            tiles.insert(coords.extend(level), TileType::Floor);
        }
    }

    room
}

pub fn spawn_room(
    commands: &mut Commands,
    grid: &Grid,
    images: &TileImages,
    tiles: &Tiles,
    level: i32,
    room: &Room,
) {
    for coords in room.coords() {
        let tile = tiles.get(&coords.extend(level)).unwrap();

        let translation = grid.map_to_world(coords);

        commands
            .spawn(SpriteBundle {
                material: images.get(tile).unwrap().clone(),
                transform: Transform::from_translation(translation - Vec3::new(0.0, 0.0, 0.1)),
                sprite: Sprite {
                    size: grid.cell_size,
                    resize_mode: SpriteResizeMode::Manual,
                },
                ..Default::default()
            })
            .with(Despawn);
    }
}

fn despawn(commands: &mut Commands, active_entities: &mut Query<Entity, With<Despawn>>) {
    for entity in active_entities.iter() {
        commands.despawn(entity);
    }
}
