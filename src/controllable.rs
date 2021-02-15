use bevy::prelude::*;

use crate::shared::{events::AiTickEvent, grid::{Coords}, math::Vec2i, moveable::Moveable, tween::Tween};

pub struct Controllable;

pub struct ControllablePlugin;
impl Plugin for ControllablePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(movement.system());
    }
}

fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Coords, &mut Moveable, &Tween), With<Controllable>>,
    mut events: ResMut<Events<AiTickEvent>>,
) {
    let direction = get_input_direction(&input);

    if direction == Vec2i::zero() {
        return;
    }
    
    for (coords, mut moveable, tween) in query.iter_mut() {
        if !tween.finished() {
            continue;
        }
        
        moveable.to = coords.0 + direction;
        events.send(AiTickEvent);
        println!("Event Send!");
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
