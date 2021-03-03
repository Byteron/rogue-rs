use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::{action::{Action, Actions}, bob::Facing};

pub struct Controllable;

pub fn input(
    input: Res<Input<KeyCode>>,
    mut actions: ResMut<Actions>,
    players: Query<(Entity, &Facing), With<Controllable>>,
) {
    for (entity, facing) in players.iter() {
        if actions.is_locked() {
            return;
        }

        let direction = get_input_direction(&input);

        if direction != Vec2i::zero() && facing.direction == direction {
            actions.queue(Action::Move(entity, direction));
            actions.lock_and_tick();
        } else if direction != Vec2i::zero() {
            actions.queue(Action::Face(entity, direction));
            actions.lock();
        } else if input.pressed(KeyCode::F) {
            actions.queue(Action::PlayerAttack(entity));
            actions.lock_and_tick();
        } else if input.pressed(KeyCode::W) {
            actions.queue(Action::Wait);
            actions.lock_and_tick();
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
