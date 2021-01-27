use bevy::prelude::*;

use crate::{
    enemies::{Enemies, EnemyType, View},
    grid::{Grid, Vec2i, Vec3i},
    player::{self, Player},
    rogue::GameState,
    tween::{Tween, TweenMode},
};

pub struct CombatEvent {
    attacker: Entity,
    defender: Entity,
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

    pub fn is_dead(&self) -> bool {
        self.value == 0
    }
}

pub struct Strength(pub i32);

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<CombatEvent>()
            .add_system(combat.system())
            .add_system_to_stage(stage::POST_UPDATE, on_combat.system())
            .add_system_to_stage(stage::LAST, death.system());
    }
}

fn combat(
    input: Res<Input<KeyCode>>,
    grid: Res<Grid>,
    state: Res<GameState>,
    enemies: Res<Enemies>,
    mut events: ResMut<Events<CombatEvent>>,
    mut players: Query<(Entity, &mut Vec2i, &mut Tween), With<Player>>,
) {
    for (player, coords, mut tween) in players.iter_mut() {
        if !tween.finished() {
            continue;
        }

        let direction = player::get_input_direction(&input);

        if direction == Vec2i::zero() {
            continue;
        }

        let from_coords = *coords;
        let to_coords = *coords + direction;

        if let Some(enemy) = enemies.0.get(&to_coords.extend(state.current_level)) {
            tween.from = grid.map_to_world(from_coords);
            tween.to = grid.map_to_world(to_coords);
            tween.start(0.15, TweenMode::Attack);

            events.send(CombatEvent {
                attacker: player,
                defender: *enemy,
            });
        }
    }
}

fn on_combat(mut events: EventReader<CombatEvent>, mut query: Query<(&mut Health, &Strength)>) {
    for event in events.iter() {
        let mut damage = 0;

        if let Ok(strength) = query.get_component::<Strength>(event.attacker) {
            damage = strength.0;
        }

        if let Ok(mut health) = query.get_component_mut::<Health>(event.defender) {
            health.hurt(damage);
        }
    }
}

fn death(
    commands: &mut Commands,
    mut enemies: ResMut<Enemies>,
    query: Query<(Entity, &Health, &View, &Vec3i), With<EnemyType>>,
) {
    for (entity, health, view, coords) in query.iter() {
        if !health.is_dead() {
            continue;
        }

        commands.despawn(entity);

        if let Some(entity) = view.0 {
            commands.despawn(entity);
        }

        enemies.0.remove(coords);
    }
}
