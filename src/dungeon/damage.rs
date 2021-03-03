use bevy::{prelude::*, utils::HashMap};

use crate::core::math::Vec2i;

use super::bob::Coords;

pub struct AttackEvent {
    pub entity: Entity,
    pub position: Vec2i,
}

pub struct DamageEvent {
    pub source: Entity,
    pub target: Entity,
}

pub struct Damage {
    pub amount: i32,
}

impl Damage {
    pub fn new(amount: i32) -> Self {
        Damage { amount }
    }
}

pub struct Damageable {
    max: i32,
    current: i32,
}

impl Damageable {
    pub fn new(health: i32) -> Self {
        Damageable {
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

    pub fn damage(&mut self, damage: i32) {
        self.current -= damage;
        if self.current <= 0 {
            self.current = 0;
        }
    }

    pub fn recover(&mut self, amount: i32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }

    pub fn extend(&mut self, amount: i32) {
        self.max += amount;
    }

    pub fn reduce(&mut self, amount: i32) {
        self.max -= amount;
        if self.max < 1 {
            self.max = 1;
        }
    }

    pub fn restore(&mut self) {
        self.current = self.max;
    }

    pub fn raze(&mut self) {
        self.current = 0;
    }

    pub fn is_full(&self) -> bool {
        self.current == self.max
    }

    pub fn is_empty(&self) -> bool {
        self.current == 0
    }
}

pub struct AttackState {
    map: HashMap<Vec2i, Entity>,
}

impl AttackState {
    pub fn has(&self, coords: Vec2i) -> bool {
        self.map.contains_key(&coords)
    }

    pub fn get(&self, coords: Vec2i) -> Option<&Entity> {
        self.map.get(&coords)
    }

    fn insert(&mut self, coords: Vec2i, entity: Entity) {
        self.map.insert(coords, entity);
    }

    fn remove(&mut self, coords: Vec2i) {
        self.map.remove(&coords);
    }

    fn clear(&mut self) {
        self.map.clear();
    }
}

impl Default for AttackState {
    fn default() -> Self {
        AttackState {
            map: HashMap::default(),
        }
    }
}

pub fn damage(
    mut event_reader: EventReader<DamageEvent>,
    mut damageables: Query<&mut Damageable>,
    damages: Query<&Damage>,
) {
    for event in event_reader.iter() {
        if let Ok(mut damageable) = damageables.get_mut(event.target) {
            if let Ok(damage) = damages.get(event.source) {
                damageable.damage(damage.amount);

                println!(
                    "{:?} damaged {:?} for {}",
                    event.source, event.target, damage.amount
                );
            }
        }
    }
}

pub fn update_state(
    mut state: ResMut<AttackState>,
    damageables: Query<(Entity, &Coords), With<Damageable>>,
) {
    state.clear();

    for (entity, coords) in damageables.iter() {
        state.insert(coords.0, entity);
    }
}

pub fn death(commands: &mut Commands, damageables: Query<(Entity, &Damageable)>) {
    for (entity, damageable) in damageables.iter() {
        if damageable.is_empty() {
            commands.despawn(entity);
        }
    }
}
