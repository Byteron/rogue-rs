use crate::core::math::Vec2i;
use bevy::prelude::*;

use super::{bob::Coords, view::ViewAnchor};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Attitude {
    Friendly,
    Neutral,
    Hostile,
}

pub struct Strength(pub i32);

pub struct Attack {
    pub direction: Vec2i,
}

pub struct Health {
    max: i32,
    value: i32,
}

impl Health {
    pub fn new(health: i32) -> Self {
        Health {
            max: health,
            value: health,
        }
    }

    pub fn hurt(&mut self, damage: i32) {
        self.value -= damage;
        if self.value <= 0 {
            self.value = 0;
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.value += amount;
        if self.value > self.max {
            self.value = self.max;
        }
    }

    pub fn increase(&mut self, amount: i32) {
        self.max += amount;
    }

    pub fn is_full(&self) -> bool {
        self.value == self.max
    }

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }
}

#[derive(Bundle)]
pub struct CombatBundle {
    pub health: Health,
    pub attack: Attack,
    pub strength: Strength,
    pub attitude: Attitude,
}

impl CombatBundle {
    pub fn new(health: i32, strength: i32, attitude: Attitude) -> Self {
        CombatBundle {
            health: Health::new(health),
            attack: Attack {
                direction: Vec2i::zero(),
            },
            strength: Strength(strength),
            attitude: attitude,
        }
    }
}

impl Default for CombatBundle {
    fn default() -> Self {
        CombatBundle {
            health: Health::new(20),
            attack: Attack {
                direction: Vec2i::zero(),
            },
            strength: Strength(12),
            attitude: Attitude::Hostile,
        }
    }
}

pub fn attack(
    mut attackers: Query<(&mut Attack, &Strength, &Coords), Mutated<Attack>>,
    mut actors: Query<(&Coords, &mut Health)>,
) {
    for (mut attack, strength, coords) in attackers.iter_mut() {
        let target_coords = Coords(coords.0 + attack.direction);

        for (other_coords, mut health) in actors.iter_mut() {
            if target_coords.0 == other_coords.0 {
                let value = health.value;
                health.hurt(strength.0);
                println!(
                    "Attack at {:?}, dealt {} Damage. ({} -> {})",
                    target_coords.0, strength.0, value, health.value
                );
            }
        }

        attack.direction = Vec2i::zero();
    }
}

pub fn death(commands: &mut Commands, query: Query<(Entity, &ViewAnchor, &Health)>) {
    for (entity, anchor, health) in query.iter() {
        if !health.is_empty() {
            continue;
        }

        if let Some(view) = anchor.0 {
            commands.despawn(view);
        }

        commands.despawn(entity);

        println!("{:?} Died", entity);
    }
}
