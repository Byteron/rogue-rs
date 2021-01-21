use bevy::prelude::*;
use dungeon::ExitRoomEvent;

use crate::{
    core::{Coordinates, Grid},
    dungeon,
    dungeon::{BoardObject, GameState, TileType},
    tween::{Tween, TweenMode},
};

pub struct Player;

pub fn movement(
    input: Res<Input<KeyCode>>,
    grid: Res<Grid>,
    mut events: ResMut<Events<ExitRoomEvent>>,
    mut state: ResMut<GameState>,
    mut players: Query<(&mut Coordinates, &mut Tween), With<Player>>,
) {
    for (mut coords, mut tween) in players.iter_mut() {
        if !tween.finished() {
            continue;
        }

        let direction = get_input_direction(&input);

        if direction == Coordinates::zero() {
            continue;
        }

        let room = state.get_current_room();

        let from_coords = *coords;
        let to_coords = *coords + direction;

        if room.is_exit(to_coords) {
            events.send(ExitRoomEvent { direction });

            tween.from = grid.map_to_world(from_coords);
            tween.to = grid.map_to_world(from_coords);
            tween.start(0.15, TweenMode::Move);
        } else if room.objects.get(&to_coords) == None {
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

pub fn combat(
    input: Res<Input<KeyCode>>,
    grid: Res<Grid>,
    mut state: ResMut<GameState>,
    mut players: Query<(&Coordinates, &mut Tween), With<Player>>,
) {
    for (coords, mut tween) in players.iter_mut() {
        if !tween.finished() {
            continue;
        }

        let direction = get_input_direction(&input);

        if direction == Coordinates::zero() {
            continue;
        }

        let room = state.get_current_room();

        let from_coords = *coords;
        let to_coords = *coords + direction;

        if let Some(bobs) = room.objects.get(&to_coords) {
            for bob in bobs {
                if let BoardObject::Enemy(enemy) = bob {
                    tween.from = grid.map_to_world(from_coords);
                    tween.to = grid.map_to_world(to_coords);
                    tween.start(0.15, TweenMode::Attack);
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
