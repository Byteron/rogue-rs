use bevy::prelude::*;

use crate::shared::{events::AiTickEvent, math::Vec2i, moveable::Moveable, tween::Tween};
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Goblin
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(goblin_ai.system());
    }
}

fn goblin_ai(
    mut event_reader: EventReader<AiTickEvent>,
    mut query: Query<(&mut Moveable, &EnemyType, &Tween)>,
) {
    let mut rng = rand::thread_rng();

    for event in event_reader.iter() {
        for (mut moveable, enemy, tween) in query.iter_mut() {
            if !tween.finished() {
                continue;
            }

            if *enemy == EnemyType::Goblin {
                let x = rng.gen_range(-1..2);
                let mut y = 0;

                if x == 0 {
                    y = rng.gen_range(-1..2);
                }

                moveable.to += Vec2i::new(x, y);
            }
        }
    }
}