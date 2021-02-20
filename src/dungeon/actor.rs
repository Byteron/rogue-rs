use bevy::prelude::*;

use crate::core::math::Vec2i;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorType {
    Human,
    Goblin,
}

pub struct Facing {
    pub direction: Vec2i,
}

pub struct ActionTimer(pub Timer);

#[derive(Bundle)]
pub struct ActorBundle {
    actor_type: ActorType,
    facing: Facing,
    action_timer: ActionTimer,
}

impl ActorBundle {
    pub fn new(actor_type: ActorType) -> Self {
        ActorBundle {
            actor_type,
            facing: Facing {
                direction: Vec2i::new(0, -1),
            },
            action_timer: ActionTimer(Timer::from_seconds(0.2, false)),
        }
    }
}

impl Default for ActorBundle {
    fn default() -> Self {
        ActorBundle {
            actor_type: ActorType::Human,
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
