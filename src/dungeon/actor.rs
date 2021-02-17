use bevy::prelude::*;

use crate::core::math::Vec2i;

pub struct Actor;

pub struct Facing {
    pub direction: Vec2i,
}
pub struct ActionTimer(pub Timer);

#[derive(Bundle)]
pub struct ActorBundle {
    actor: Actor,
    facing: Facing,
    action_timer: ActionTimer,
}

impl Default for ActorBundle {
    fn default() -> Self {
        ActorBundle {
            actor: Actor,
            facing: Facing {
                direction: Vec2i::new(0, -1),
            },
            action_timer: ActionTimer(Timer::from_seconds(0.2, false)),
        }
    }
}

pub fn tick(time: Res<Time>, mut query: Query<&mut ActionTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta_seconds());
    }
}
