use bevy::{prelude::*, render::camera::Camera};

use rand::Rng;

use crate::{
    enemies::{Enemies, EnemyImages, EnemyType},
    grid::{Grid, Vec2i, Vec3i},
    rogue::GameState,
    rooms::{self, Room, Rooms, TileImages, TileType, Tiles},
};

pub struct Despawn;

pub struct RoomExitedEvent {
    pub direction: Vec2i,
}

pub struct RoomEnteredEvent;

pub fn generate(rooms: &mut Rooms, tiles: &mut Tiles, enemies: &mut Enemies) -> Vec3i {
    let room_size = Vec2i::new(19, 11);
    let level_size = Vec2i::new(10, 10);

    for level in 0..50 {
        for y in -level_size.y..=level_size.y {
            for x in -level_size.x..=level_size.x {
                let room_coords = Vec2i::new(x, y);

                let room = rooms::create_room(tiles, level, room_coords * room_size, room_size);
                rooms.0.insert(room_coords.extend(level), room);
            }
        }
    }

    let mut rng = rand::thread_rng();

    for (room_coords, room) in rooms.0.iter() {
        for coords in room.coords() {
            let tile = tiles.0.get(&coords.extend(room_coords.z)).unwrap();
            if *tile == TileType::Floor {
                if rng.gen_bool(0.01) && !room.is_door(coords) {
                    enemies
                        .0
                        .insert(coords.extend(room_coords.z), EnemyType::Goblin);
                }
            }
        }
    }

    (level_size / Vec2i::new(2, 2)).extend(0)
}

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<RoomExitedEvent>()
            .add_event::<RoomEnteredEvent>()
            .add_system(on_room_exited.system())
            .add_system(on_room_entered.system());
    }
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

        if let Some(_) = rooms.0.get(&new_room) {
            despawn(commands, &mut active_entities);
            state.current_room = new_room;
            events.send(RoomEnteredEvent);
        }
    }
}

fn on_room_entered(
    commands: &mut Commands,
    tile_images: Res<TileImages>,
    enemy_images: Res<EnemyImages>,
    grid: Res<Grid>,
    rooms: Res<Rooms>,
    tiles: Res<Tiles>,
    enemies: Res<Enemies>,
    state: Res<GameState>,
    mut events: EventReader<RoomEnteredEvent>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    for _ in events.iter() {
        let mut camera_transform = cameras.iter_mut().next().unwrap();
        let room = rooms.0.get(&state.current_room).unwrap();

        spawn_room(
            commands,
            &grid,
            &tile_images,
            &enemy_images,
            &tiles,
            &enemies,
            state.current_level,
            &room,
        );
        camera_transform.translation = grid.map_to_world(room.center());
    }
}

pub fn spawn_room(
    commands: &mut Commands,
    grid: &Grid,
    tile_images: &TileImages,
    enemy_images: &EnemyImages,
    tiles: &Tiles,
    enemies: &Enemies,
    level: i32,
    room: &Room,
) {
    for coords in room.coords() {
        let tile = tiles.0.get(&coords.extend(level)).unwrap();

        spawn_tile(
            commands,
            grid,
            coords,
            tile_images.0.get(tile).unwrap().clone(),
        );

        if let Some(enemy) = enemies.0.get(&coords.extend(level)) {
            spawn_enemy(
                commands,
                grid,
                coords,
                enemy_images.0.get(enemy).unwrap().clone(),
            )
        }
    }
}

pub fn spawn_tile(
    commands: &mut Commands,
    grid: &Grid,
    coords: Vec2i,
    material: Handle<ColorMaterial>,
) {
    let translation = grid.map_to_world(coords);

    commands
        .spawn(SpriteBundle {
            material,
            transform: Transform::from_translation(translation - Vec3::new(0.0, 0.0, 0.1)),
            sprite: Sprite {
                size: grid.cell_size,
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(coords)
        .with(Despawn);
}

pub fn spawn_enemy(
    commands: &mut Commands,
    grid: &Grid,
    coords: Vec2i,
    material: Handle<ColorMaterial>,
) {
    let translation = grid.map_to_world(coords);

    commands
        .spawn(SpriteBundle {
            material,
            transform: Transform::from_translation(translation - Vec3::new(0.0, 0.0, 0.1)),
            sprite: Sprite {
                size: grid.cell_size,
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .with(EnemyType::Goblin)
        .with(coords)
        .with(Despawn);
}

fn despawn(commands: &mut Commands, active_entities: &mut Query<Entity, With<Despawn>>) {
    for entity in active_entities.iter() {
        commands.despawn(entity);
    }
}
