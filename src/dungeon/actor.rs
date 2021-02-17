use bevy::prelude::*;

use crate::core::math::Vec2i;

pub struct Actor;

pub struct Approach {
    pub direction: Vec2i,
}

pub struct ApproachTimer(pub Timer);

#[derive(Bundle)]
pub struct ActorBundle {
    actor: Actor,
    approach: Approach,
    approach_timer: ApproachTimer,
}

impl Default for ActorBundle {
    fn default() -> Self {
        ActorBundle {
            actor: Actor,
            approach: Approach {
                direction: Vec2i::zero(),
            },
            approach_timer: ApproachTimer(Timer::from_seconds(0.15, false)),
        }
    }
}

pub fn tick(time: Res<Time>, mut query: Query<&mut ApproachTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta_seconds());
    }
}

pub fn cleanup(mut query: Query<&mut Approach, Mutated<Approach>>) {
    for mut approach in query.iter_mut() {
        approach.direction = Vec2i::zero();
    }
}
