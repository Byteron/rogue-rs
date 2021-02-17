use bevy::prelude::*;

use crate::core::math::Vec2i;

pub struct Actor;

pub struct Approach {
    pub direction: Vec2i,
}

#[derive(Bundle)]
pub struct ActorBundle {
    actor: Actor,
    approach: Approach,
}

impl Default for ActorBundle {
    fn default() -> Self {
        ActorBundle {
            actor: Actor,
            approach: Approach {
                direction: Vec2i::zero(),
            },
        }
    }
}
