use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::actor::Approach;

pub struct Player;

pub fn movement(input: Res<Input<KeyCode>>, mut players: Query<&mut Approach, With<Player>>) {
    for mut approach in players.iter_mut() {
        let direction = get_input_direction(&input);

        if direction == Vec2i::zero() {
            continue;
        }

        approach.direction = direction;
        println!("Approaching {:?}", direction);
    }
}

fn get_input_direction(input: &Input<KeyCode>) -> Vec2i {
    let mut direction = Vec2i::zero();

    if input.just_pressed(KeyCode::W) || input.just_pressed(KeyCode::Up) {
        direction.y += 1;
    } else if input.just_pressed(KeyCode::S) || input.just_pressed(KeyCode::Down) {
        direction.y -= 1;
    } else if input.just_pressed(KeyCode::D) || input.just_pressed(KeyCode::Right) {
        direction.x += 1;
    } else if input.just_pressed(KeyCode::A) || input.just_pressed(KeyCode::Left) {
        direction.x -= 1;
    }

    direction
}
