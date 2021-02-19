use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::{
    actor::{ActionTimer, Facing},
    ai::AiTickEvent,
    combat::Attack,
    physics::Step,
};

pub struct Player;

pub fn input(
    input: Res<Input<KeyCode>>,
    mut events: ResMut<Events<AiTickEvent>>,
    mut query: Query<(&mut Step, &mut Attack, &mut ActionTimer, &mut Facing), With<Player>>,
) {
    for (mut step, mut attack, mut timer, mut facing) in query.iter_mut() {
        if !timer.0.finished() {
            continue;
        }

        if input.pressed(KeyCode::F) {
            attack.direction = facing.direction;
            timer.0.reset();
            events.send(AiTickEvent);
            return;
        }

        let direction = get_input_direction(&input);

        if !timer.0.finished() || direction == Vec2i::zero() {
            continue;
        }

        if facing.direction == direction {
            step.direction = direction;
            timer.0.reset();
            events.send(AiTickEvent);
        } else {
            facing.direction = direction;
            timer.0.reset();
            println!("Faced to {:?}", direction);
        }
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
