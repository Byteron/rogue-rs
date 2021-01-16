use bevy::prelude::*;

use crate::{
    core::{Coordinates, Grid},
    rogue::Room,
};

pub struct Player;

pub struct StepTimer(pub Timer);

pub fn step(
    grid: Res<Grid>,
    mut query: Query<(&mut Transform, &Coordinates), (With<Player>, Changed<Coordinates>)>,
) {
    for (mut transform, coords) in query.iter_mut() {
        transform.translation = grid.map_to_world(*coords);
    }
}

pub fn input(
    input: Res<Input<KeyCode>>,
    room: Res<Room>,
    time: Res<Time>,
    mut step_timer: ResMut<StepTimer>,
    mut query: Query<&mut Coordinates, With<Player>>,
) {
    if !step_timer.0.finished() {
        step_timer.0.tick(time.delta_seconds());
        return;
    }

    for mut coords in query.iter_mut() {
        let direction = get_input_direction(&input);

        if direction != Coordinates::zero() {
            if let Some(tile) = room.tiles.get(&(*coords + direction)) {
                match tile {
                    crate::rogue::Tile::Wall => {}
                    crate::rogue::Tile::Floor => {
                        *coords += direction;
                        step_timer.0.reset();
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
