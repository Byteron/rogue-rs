use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::{actor::StepTimer, grid::Coords};
pub struct Player;

pub fn movement(
    input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Coords, &mut StepTimer), With<Player>>,
) {
    for (mut coords, mut timer) in players.iter_mut() {
        if !timer.0.finished() {
            continue;
        }

        let direction = get_input_direction(&input);

        if direction == Vec2i::zero() {
            continue;
        }

        coords.0 += direction;
        timer.0.reset();
    }
}

fn get_input_direction(input: &Input<KeyCode>) -> Vec2i {
    let mut direction = Vec2i::zero();

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
