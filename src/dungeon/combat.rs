use crate::core::math::Vec2i;
use bevy::prelude::*;

use super::bob::Coords;

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
    current: i32,
}

impl Health {
    pub fn new(health: i32) -> Self {
        Health {
            max: health,
            current: health,
        }
    }

    pub fn current(&self) -> i32 {
        self.current
    }

    pub fn max(&self) -> i32 {
        self.max
    }

    pub fn hurt(&mut self, damage: i32) {
        self.current -= damage;
        if self.current <= 0 {
            self.current = 0;
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }

    pub fn increase(&mut self, amount: i32) {
        self.max += amount;
    }

    pub fn is_full(&self) -> bool {
        self.current == self.max
    }

    pub fn is_empty(&self) -> bool {
        self.current == 0
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

pub fn death(commands: &mut Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if !health.is_empty() {
            continue;
        }

        commands.despawn(entity);

        println!("{:?} Died", entity);
    }
}
