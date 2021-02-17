use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::bob::Coords;

pub struct Body {
    solid: bool,
}

impl Body {
    pub fn new(solid: bool) -> Self {
        Body { solid }
    }

    pub fn solid() -> Self {
        Body { solid: true }
    }

    pub fn hollow() -> Self {
        Body { solid: true }
    }
}

pub struct StepTimer(pub Timer);

pub struct Step {
    pub direction: Vec2i,
}

impl Default for Step {
    fn default() -> Self {
        Step {
            direction: Vec2i::zero(),
        }
    }
}

#[derive(Bundle)]
pub struct KinematicBodyBundle {
    pub step: Step,
    pub step_timer: StepTimer,
    pub body: Body,
}

impl KinematicBodyBundle {
    pub fn new(solid: bool) -> Self {
        KinematicBodyBundle {
            step: Step::default(),
            step_timer: StepTimer(Timer::from_seconds(0.15, false)),
            body: Body { solid },
        }
    }

    pub fn solid() -> Self {
        KinematicBodyBundle {
            step: Step::default(),
            step_timer: StepTimer(Timer::from_seconds(0.15, false)),
            body: Body::solid(),
        }
    }

    pub fn hollow() -> Self {
        KinematicBodyBundle {
            step: Step::default(),
            step_timer: StepTimer(Timer::from_seconds(0.15, false)),
            body: Body::hollow(),
        }
    }
}

pub fn collision(
    mut movers: Query<(&mut Step, &Coords, &Body), Mutated<Step>>,
    bodies: Query<(&Coords, &Body)>,
) {
    for (mut step, coords, body) in movers.iter_mut() {
        if !body.solid {
            continue;
        }

        let target_coords = Coords(coords.0 + step.direction);

        for (other_coords, other_body) in bodies.iter() {
            if other_body.solid && target_coords.0 == other_coords.0 {
                step.direction = Vec2i::zero();
                break;
            }
        }
    }
}

pub fn step(time: Res<Time>, mut query: Query<(&Step, &mut StepTimer, &mut Coords), With<Body>>) {
    for (step, mut timer, mut coords) in query.iter_mut() {
        timer.0.tick(time.delta_seconds());

        if timer.0.finished() && step.direction != Vec2i::zero() {
            coords.0 += step.direction;
            timer.0.reset();
        }
    }
}

pub fn cleanup(mut query: Query<&mut Step, Changed<Step>>) {
    for mut step in query.iter_mut() {
        step.direction = Vec2i::zero();
    }
}
