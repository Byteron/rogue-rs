use bevy::prelude::*;
use dungeon::ExitRoomEvent;

use crate::{
    core::{Coordinates, Grid},
    dungeon,
    dungeon::{BoardObject, GameState, TileType},
    tween::{Tween, TweenMode},
};

pub struct Player;

pub fn input(
    input: Res<Input<KeyCode>>,
    grid: Res<Grid>,
    time: Res<Time>,
    mut events: ResMut<Events<ExitRoomEvent>>,
    mut state: ResMut<GameState>,
    mut players: Query<(&mut Coordinates, &mut Tween), With<Player>>,
) {
    for (mut coords, mut tween) in players.iter_mut() {
        if !tween.finished() {
            tween.tick(time.delta_seconds());
            continue;
        }

        let room = state.get_current_room();
        let direction = get_input_direction(&input);

        if direction != Coordinates::zero() {
            let from_coords = *coords;
            let to_coords = *coords + direction;

            if room.is_exit(to_coords) {
                events.send(ExitRoomEvent { direction });

                tween.from = grid.map_to_world(from_coords);
                tween.to = grid.map_to_world(from_coords);
                tween.start(0.15, TweenMode::Move);
            } else {
                match room.objects.get(&to_coords) {
                    Some(bobs) => {
                        handle_bobs(&grid, bobs, from_coords, to_coords, &mut tween);
                    }
                    None => {
                        if let Some(tile) = room.tiles.get(&to_coords) {
                            if *tile == TileType::Floor {
                                tween.from = grid.map_to_world(from_coords);
                                tween.to = grid.map_to_world(to_coords);
                                tween.start(0.15, TweenMode::Move);

                                *coords = to_coords;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn handle_bobs(
    grid: &Grid,
    bobs: &Vec<BoardObject>,
    from_coords: Coordinates,
    to_coords: Coordinates,
    tween: &mut Tween,
) {
    for bob in bobs {
        match bob {
            BoardObject::Player => {}
            BoardObject::Enemy(enemy) => {
                tween.from = grid.map_to_world(from_coords);
                tween.to = grid.map_to_world(to_coords);
                tween.start(0.15, TweenMode::Attack);
            }
            BoardObject::Item(item) => {
                // item collection logic
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
