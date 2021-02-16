use bevy::prelude::*;

pub struct Actor;

pub struct StepTimer(pub Timer);

#[derive(Bundle)]
pub struct ActorBundle {
    actor: Actor,
    step_timer: StepTimer,
}

impl Default for ActorBundle {
    fn default() -> Self {
        ActorBundle {
            actor: Actor,
            step_timer: StepTimer(Timer::from_seconds(0.2, false)),
        }
    }
}

pub fn step_timer_tick(time: Res<Time>, mut query: Query<&mut StepTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta_seconds());
    }
}
