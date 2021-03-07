use bevy::prelude::*;

use super::{
    action::{Action, Actions},
    bob::Facing,
};

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

        if direction != IVec2::ZERO && facing.direction == direction {
            actions.queue(Action::Move(entity, direction));
            actions.lock_and_tick();
        } else if direction != IVec2::ZERO {
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

fn get_input_direction(input: &Input<KeyCode>) -> IVec2 {
    let mut direction = IVec2::ZERO;

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
