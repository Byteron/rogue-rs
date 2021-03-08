use bevy::{prelude::*, utils::HashMap};

use super::{bob::{Layer, Position}, grid::Grid, tween::{Tween, TweenMode}};

pub struct AttackEvent {
    pub entity: Entity,
    pub position: IVec2,
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
    map: HashMap<IVec2, Entity>,
}

impl AttackState {
    pub fn has(&self, coords: IVec2) -> bool {
        self.map.contains_key(&coords)
    }

    pub fn get(&self, coords: IVec2) -> Option<&Entity> {
        self.map.get(&coords)
    }

    fn insert(&mut self, coords: IVec2, entity: Entity) {
        self.map.insert(coords, entity);
    }

    fn remove(&mut self, coords: IVec2) {
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
    grid: Res<Grid>,
    mut event_reader: EventReader<DamageEvent>,
    mut damageables: Query<(&mut Damageable, &Position, &Layer)>,
    mut damages: Query<(&Damage, &Position, &Layer, &mut Tween)>,
) {
    for event in event_reader.iter() {
        if let Ok((mut damageable, defender_position, defender_layer)) = damageables.get_mut(event.target) {
            if let Ok((damage, attacker_position, attacker_layer, mut tween)) = damages.get_mut(event.source) {
                damageable.damage(damage.amount);
                
                tween.from = grid.map_to_world(attacker_position.0).extend(attacker_layer.0).as_f32();
                tween.to = grid.map_to_world(defender_position.0).extend(defender_layer.0).as_f32();
                tween.start(0.2, TweenMode::Attack);
                
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
    damageables: Query<(Entity, &Position), With<Damageable>>,
) {
    state.clear();

    for (entity, coords) in damageables.iter() {
        state.insert(coords.0, entity);
    }
}

pub fn death(mut commands: Commands, damageables: Query<(Entity, &Damageable)>) {
    for (entity, damageable) in damageables.iter() {
        if damageable.is_empty() {
            commands.despawn(entity);
        }
    }
}
