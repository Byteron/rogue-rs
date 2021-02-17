use bevy::prelude::*;
use rand::Rng;

use crate::core::math::Vec2i;

use super::physics::Step;

pub struct AiTickEvent;

pub struct GoblinAi;

pub fn goblin_ai_movement(
    mut event_reader: EventReader<AiTickEvent>,
    mut query: Query<&mut Step, With<GoblinAi>>,
) {
    let mut rng = rand::thread_rng();

    for event in event_reader.iter() {
        for mut step in query.iter_mut() {
            let x = rng.gen_range(-1..=1);
            let mut y = 0;

            if x == 0 {
                y = rng.gen_range(-1..=1);
            }

            step.direction = Vec2i::new(x, y);
        }
    }
}
