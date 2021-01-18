use bevy::{prelude::*, render::camera::Camera};

use crate::{core::{Coordinates, Grid, Stepper}, dungeon, dungeon::{Level, TileType, Images}};

pub struct Player;

pub fn input(
    commands: &mut Commands,
    images: Res<Images>,
    input: Res<Input<KeyCode>>,
    grid: Res<Grid>,
    time: Res<Time>,
    mut level: ResMut<Level>,
    mut query: Query<(&mut Coordinates, &mut Stepper), With<Player>>,
    mut cameras: Query<&mut Transform, With<Camera>>
) {
    for (mut coords, mut stepper) in query.iter_mut() {
        if !stepper.finished() {
            stepper.tick(time.delta_seconds());
            continue;
        }

        let direction = get_input_direction(&input);

        if direction != Coordinates::zero() {
            let from = *coords;
            let to = *coords + direction;
            
            if level.get_current_room().is_exit(to) {
                
                dungeon::despawn_room(commands, level.get_current_room());
                
                level.change_current(direction);
                
                let mut camera_transform = cameras.get_mut(level.camera).unwrap();

                let room = level.get_current_room();

                dungeon::spawn_room(commands, &grid, &images, room);
                
                camera_transform.translation = grid.map_to_world(room.center());

                stepper.from = grid.map_to_world(from);
                stepper.to = grid.map_to_world(from);

                stepper.reset();
                
            }
            else if let Some(tile) = level.get_current_room().tiles.get(&(to)) {
                match tile {
                    TileType::Wall => {
                        // nothing
                    }
                    TileType::Floor => {
                        stepper.from = grid.map_to_world(from);
                        stepper.to = grid.map_to_world(to);
                        stepper.reset();

                        *coords = to;
                    }
                }
            }
        }
    }
}

fn get_input_direction(input: &Input<KeyCode>) -> Coordinates {
    let mut direction = Coordinates::zero();

    if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
        direction.y += 1;
    } else if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
        direction.y -= 1;
    } else if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
        direction.x += 1;
    } else if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
        direction.x -= 1;
    }

    direction
}
