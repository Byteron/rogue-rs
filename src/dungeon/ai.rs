use std::time::Instant;

use bevy::{prelude::*, utils::HashSet};
use rand::Rng;

use crate::core::math::Vec2i;

use super::{bob::Coords, combat::Attack, physics::Step, player::Player};

pub struct AiTickEvent;

pub struct GoblinAi;

pub fn goblin_ai_movement(
    mut event_reader: EventReader<AiTickEvent>,
    mut query: Query<(&mut Step, &mut Attack, &Coords), With<GoblinAi>>,
    players: Query<&Coords, With<Player>>,
) {
    let start = Instant::now();

    let mut rng = rand::thread_rng();

    let mut attack_coordinates: HashSet<Vec2i> = HashSet::default();

    for coords in players.iter() {
        attack_coordinates.insert(coords.0);
    }

    for _ in event_reader.iter() {
        'goblin: for (mut step, mut attack, goblin_coords) in query.iter_mut() {
            for (n_dir, n_coords) in goblin_coords.get_neighbors().iter() {
                if attack_coordinates.contains(&n_coords.0) {
                    println!("Goblin found Player!, Attacking: {:?}", n_dir);
                    attack.direction = *n_dir;
                    continue 'goblin;
                }
            }

            let x = rng.gen_range(-1..=1);
            let mut y = 0;

            if x == 0 {
                y = rng.gen_range(-1..=1);
            }

            step.direction = Vec2i::new(x, y);
        }
    }

    println!("AIFrame: {:?}", start.elapsed());
}
