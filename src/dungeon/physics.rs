use bevy::prelude::*;

use crate::core::math::Vec2i;

use super::{actor::Approach, bob::Coords};

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
    pub body: Body,
}

impl KinematicBodyBundle {
    pub fn new(solid: bool) -> Self {
        KinematicBodyBundle {
            step: Step::default(),
            body: Body { solid },
        }
    }

    pub fn solid() -> Self {
        KinematicBodyBundle {
            step: Step::default(),
            body: Body::solid(),
        }
    }

    pub fn hollow() -> Self {
        KinematicBodyBundle {
            step: Step::default(),
            body: Body::hollow(),
        }
    }
}

pub fn approach(
    mut movers: Query<(&mut Approach, &Coords, &mut Step, &Body), Mutated<Approach>>,
    bodies: Query<(&Coords, &Body)>,
) {
    for (mut approach, coords, mut step, body) in movers.iter_mut() {
        if !body.solid || approach.direction == Vec2i::zero() {
            approach.direction = Vec2i::zero();
            continue;
        }

        let mut colliding = false;

        let target_coords = Coords(coords.0 + approach.direction);

        for (other_coords, body) in bodies.iter() {
            if body.solid && target_coords.0 == other_coords.0 {
                colliding = true;
                println!("Collision at {:?}", target_coords.0);
                break;
            }
        }

        if !colliding {
            step.direction = approach.direction;
            approach.direction = Vec2i::zero();
        }
    }
}

pub fn movement(mut query: Query<(&mut Step, &mut Coords), Mutated<Step>>) {
    for (mut step, mut coords) in query.iter_mut() {
        coords.0 += step.direction;
        println!("Stepped on {:?}", coords.0);

        step.direction = Vec2i::zero();
    }
}
