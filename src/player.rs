use std::time::Duration;

use bevy::prelude::*;

use crate::{core::{Coordinates, Tween}, tile_map::TileMap};

pub struct Player;

pub fn step(mut query: Query<(&mut Transform, &Tween), (With<Player>, Changed<Tween>)>) {
    for (mut transform, tween) in query.iter_mut() {
        transform.translation = tween.value();
    }
}

pub fn input(
    input: Res<Input<KeyCode>>,
    map: Res<TileMap>,
    mut query: Query<(&mut Coordinates, &mut Tween), With<Player>>,
) {
    for (mut coords, mut tween) in query.iter_mut() {
        if !tween.finished() {
            continue;
        }

        let direction = get_input_direction(&input);

        if direction != Coordinates::zero() {
            let prev = *coords;

            *coords += direction;

            tween.tween(
                map.map_to_world(prev),
                map.map_to_world(*coords),
                Duration::from_secs_f32(0.2),
            );
        }
    }
}

fn get_input_direction(input: &Input<KeyCode>) -> Coordinates {
    let mut direction = Coordinates::zero();

    if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
        direction.y += 1;
    }

    if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
        direction.y -= 1;
    }

    if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
        direction.x += 1;
    }

    if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
        direction.x -= 1;
    }

    direction
}