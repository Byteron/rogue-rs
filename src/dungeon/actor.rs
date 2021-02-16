use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::grid::{Coords, Grid};

pub struct Actor;

pub struct StepTimer(pub Timer);

#[derive(Bundle)]
pub struct ActorBundle {
    actor: Actor,
    coords: Coords,
    transform: Transform,
    step_timer: StepTimer,
}

impl Default for ActorBundle {
    fn default() -> Self {
        ActorBundle {
            actor: Actor,
            coords: Coords(Vec2i::zero()),
            transform: Transform::default(),
            step_timer: StepTimer(Timer::from_seconds(0.2, false)),
        }
    }
}

pub fn step_timer_tick(time: Res<Time>, mut query: Query<&mut StepTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta_seconds());
    }
}

pub fn update_position(grid: Res<Grid>, mut query: Query<(&mut Transform, &Coords), (With<Actor>, Changed<Coords>)>) {
    for (mut transform, coords) in query.iter_mut() {
        transform.translation = grid.map_to_world(coords.0).extend(0).as_f32();
    }
}
