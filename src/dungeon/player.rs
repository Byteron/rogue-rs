use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::actor::{Approach, ApproachTimer};

pub struct Player;

pub fn movement(
    input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Approach, &mut ApproachTimer), With<Player>>,
) {
    for (mut approach, mut timer) in players.iter_mut() {
        let direction = get_input_direction(&input);

        if !timer.0.finished() || direction == Vec2i::zero() {
            continue;
        }

        approach.direction = direction;
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
