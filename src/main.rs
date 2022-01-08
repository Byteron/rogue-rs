mod components;
mod resources;
mod systems;

use bevy::{prelude::*, utils::HashMap};
use components::*;
use rand::Rng;
use resources::*;

fn main() {
    App::new()
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
        .add_system(systems::lerp)
        .run();
}

fn setup(mut commands: Commands, settings: Res<Settings>) {
    let start_tile = settings.room_size / 2;
    let start_position = (start_tile * settings.tile_size).extend(0).as_vec3();
    let mut rng = rand::thread_rng();

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(settings.tile_size.as_vec2()),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Controllable)
        .insert(MoveTween {
            start: start_tile,
            end: start_tile,
            timer: Timer::from_seconds(0.2, false),
        })
        .insert(Coords(start_tile))
        .insert(Transform::from_translation(start_position));

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
                    let tile_position = (tile_coords * settings.tile_size).extend(0).as_vec3();

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
                        .id();

                    if room.is_wall(tile_coords) {
                        commands.entity(tile).insert(Solid);
                    }

                    tiles.0.insert((tile_coords.x, tile_coords.y, floor), tile);
                }
            }
        }
    }

    commands.insert_resource(tiles);
    commands.insert_resource(Floor { current: 0 });
}

pub struct Room {
    start: IVec2,
    end: IVec2,
}

impl Room {
    fn new(start: IVec2, end: IVec2) -> Self {
        return Room { start, end };
    }

    fn is_wall(&self, coords: IVec2) -> bool {
        return self.start.x == coords.x
            || self.end.x - 1 == coords.x
            || self.start.y == coords.y
            || self.end.y - 1 == coords.y;
    }
}
