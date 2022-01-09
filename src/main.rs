mod components;
mod events;
mod resources;
mod systems;
mod utils;

use bevy::{prelude::*, utils::HashMap};
use components::*;
use events::*;
use rand::Rng;
use resources::*;
use utils::Room;

fn main() {
    App::new()
        .add_event::<MovedEvent>()
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "ROGUE PRE-ALPHA".into(),
            ..Default::default()
        })
        .insert_resource(Settings {
            floor_count: 1,
            room_count: 1,
            room_size: IVec2::splat(13),
            tile_size: IVec2::splat(64),
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(systems::tick)
        .add_system(systems::control)
        .add_system(systems::roam)
        .add_system(systems::lerp)
        .run();
}

fn setup(mut commands: Commands, settings: Res<Settings>) {
    let mut rng = rand::thread_rng();

    let mut tiles = Tiles(HashMap::default());

    for floor in 0..settings.floor_count {
        for room in 0..settings.room_count {
            let room_coords = IVec2::new(0, room.try_into().unwrap()) * settings.room_size;
            let room_end_coords = room_coords + settings.room_size;
            let room = Room::new(room_coords, room_end_coords);

            for y in 0..settings.room_size.y {
                for x in 0..settings.room_size.x {
                    let local_tile_coords = IVec2::new(x, y);
                    let tile_coords = local_tile_coords + room_coords;
                    let tile_position = (tile_coords * settings.tile_size).as_vec2().extend(-0.04);

                    let grey: f32;
                    if room.is_wall(tile_coords) {
                        grey = rng.gen_range(0.22..0.25);
                    } else {
                        grey = rng.gen_range(0.72..0.75);
                    }

                    let tile = commands
                        .spawn()
                        .insert_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgb(grey, grey, grey),
                                custom_size: Some(settings.tile_size.as_vec2()),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(tile_position),
                            ..Default::default()
                        })
                        .insert(Coords(tile_coords))
                        .insert(CharacterSlot { entity: None })
                        .id();

                    if room.is_wall(tile_coords) {
                        commands.entity(tile).insert(Solid);
                    }

                    tiles.0.insert((tile_coords.x, tile_coords.y, floor), tile);
                }
            }
        }
    }

    let player = spawn_character(&mut commands, &tiles, IVec2::splat(4), Color::BLUE);

    commands
        .entity(player)
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Controllable);

    let enemy = spawn_character(&mut commands, &tiles, IVec2::splat(2), Color::RED);
    commands.entity(enemy).insert(Roamer);

    let enemy = spawn_character(&mut commands, &tiles, IVec2::splat(3), Color::RED);
    commands.entity(enemy).insert(Roamer);

    let enemy = spawn_character(&mut commands, &tiles, IVec2::splat(7), Color::RED);
    commands.entity(enemy).insert(Roamer);

    let enemy = spawn_character(&mut commands, &tiles, IVec2::splat(8), Color::RED);
    commands.entity(enemy).insert(Roamer);

    commands.insert_resource(tiles);
    commands.insert_resource(Floor { current: 0 });
}

fn spawn_character(
    commands: &mut Commands,
    tiles: &Tiles,
    start_tile: IVec2,
    color: Color,
) -> Entity {
    let character = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(60.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MoveTween {
            start: start_tile,
            end: start_tile,
            timer: Timer::from_seconds(0.2, false),
        })
        .insert(Coords(start_tile))
        .insert(Solid)
        .id();

    let tile = tiles.0.get(&(start_tile.x, start_tile.y, 0)).unwrap();

    commands.entity(*tile).insert(CharacterSlot {
        entity: Some(character),
    });

    character
}
