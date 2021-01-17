use bevy::prelude::*;

use crate::{
    core::{Coordinates, Grid, Stepper},
    rogue::*,
};

pub struct Player;

pub fn input(
    input: Res<Input<KeyCode>>,
    grid: Res<Grid>,
    room: Res<Room>,
    time: Res<Time>,
    mut query: Query<(&mut Coordinates, &mut Stepper), With<Player>>,
) {
    for (mut coords, mut stepper) in query.iter_mut() {
        if !stepper.timer.finished() {
            stepper.timer.tick(time.delta_seconds());
            continue;
        }

        let direction = get_input_direction(&input);

        if direction != Coordinates::zero() {
            if let Some(tile) = room.tiles.get(&(*coords + direction)) {
                match tile {
                    Tile::Wall => {
                        // nothing
                    }
                    Tile::Floor => {
                        stepper.from = grid.map_to_world(*coords);
                        *coords += direction;
                        stepper.to = grid.map_to_world(*coords);
                        stepper.timer.reset();
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
