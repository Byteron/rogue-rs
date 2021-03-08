use bevy::{prelude::*, utils::HashSet};
use rand::Rng;

use super::{
    action::{Action, Actions},
    bob::Position,
    player::Controllable,
};

pub struct TickEvent;

pub struct GoblinAi;

pub fn goblin_ai_movement(
    mut actions: ResMut<Actions>,
    mut event_reader: EventReader<TickEvent>,
    mut query: Query<(Entity, &Position), With<GoblinAi>>,
    players: Query<&Position, With<Controllable>>,
) {
    let mut rng = rand::thread_rng();

    let mut attack_coordinates: HashSet<IVec2> = HashSet::default();

    for coords in players.iter() {
        attack_coordinates.insert(coords.0);
    }

    for _ in event_reader.iter() {
        'goblin: for (goblin_entity, goblin_coords) in query.iter_mut() {
            for (n_dir, n_coords) in goblin_coords.get_neighbors().iter() {
                if attack_coordinates.contains(&n_coords.0) {
                    println!("Goblin found Player!, Attacking: {:?}", n_dir);
                    actions.push(Action::Attack(goblin_entity));
                    continue 'goblin;
                }
            }

            let x = rng.gen_range(-1..=1);
            let mut y = 0;

            if x == 0 {
                y = rng.gen_range(-1..=1);
            }

            actions.push(Action::Move(goblin_entity, IVec2::new(x, y)));
        }

        actions.queue(Action::Delay);
    }
}
