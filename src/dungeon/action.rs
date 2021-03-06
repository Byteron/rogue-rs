use std::collections::VecDeque;

use bevy::prelude::*;

use super::{ai::TickEvent, bob::{Position, Facing}, damage::{AttackState, DamageEvent}, physics::{MoveEvent, PhysicsState, Velocity}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Tick,
    Face(Entity, IVec2),
    PlayerAttack(Entity),
    Attack(Entity),
    Move(Entity, IVec2),
    Wait,
    Delay,
}

pub struct Actions {
    queue: VecDeque<Action>,
    timer: Timer,
    is_locked: bool,
}

impl Actions {
    pub fn lock_and_tick(&mut self) {
        self.lock();
        self.queue(Action::Tick);
    }

    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    pub fn queue(&mut self, action: Action) {
        self.queue.push_back(action);
    }

    pub fn push(&mut self, action: Action) {
        self.queue.push_front(action);
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    fn unlock(&mut self) {
        self.is_locked = false;
    }
}

impl Default for Actions {
    fn default() -> Self {
        Actions {
            queue: VecDeque::default(),
            timer: Timer::from_seconds(0.2, false),
            is_locked: false,
        }
    }
}

pub fn actions(
    time: Res<Time>,
    mut actions: ResMut<Actions>,
    attack_state: Res<AttackState>,
    mut move_events: EventWriter<MoveEvent>,
    mut attack_events: EventWriter<DamageEvent>,
    mut tick_events: EventWriter<TickEvent>,
    mut query: Query<(&Position, &mut Facing)>,
) {
    actions.timer.tick(time.delta());

    if !actions.timer.finished() {
        return;
    }

    if actions.timer.just_finished() && actions.queue.is_empty() {
        actions.unlock();
    }

    while !actions.queue.is_empty() {
        if let Some(action) = actions.queue.pop_front() {
            match action {
                Action::PlayerAttack(entity) => {
                    actions.push(Action::Delay);
                    actions.push(Action::Attack(entity));
                }
                Action::Attack(entity) => {
                    let (coords, facing) = query.get_mut(entity).unwrap();
                    let target_position = coords.0 + facing.direction;

                    if let Some(target) = attack_state.get(target_position) {
                        attack_events.send(DamageEvent {
                            source: entity,
                            target: *target,
                        });
                    }
                }
                Action::Move(entity, direction) => {
                    move_events.send(MoveEvent {
                        entity,
                        direction,
                    });
                }
                Action::Tick => {
                    tick_events.send(TickEvent);
                }
                Action::Wait => {
                    // Do Nothing
                }
                Action::Face(entity, direction) => {
                    let mut facing = query.get_component_mut::<Facing>(entity).unwrap();
                    facing.direction = direction;
                    actions.push(Action::Delay);
                }
                Action::Delay => {
                    actions.timer.reset();
                }
            }
        }
    }
}
