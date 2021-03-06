use bevy::{prelude::*, utils::HashMap};

use crate::core::math::Vec2i;

use super::{
    grid::Grid,
    tween::{Tween, TweenMode},
};

pub struct Coords(pub Vec2i);

pub struct Facing {
    pub direction: Vec2i,
}

impl Coords {
    pub fn get_neighbors(&self) -> [(Vec2i, Coords); 4] {
        [
            (Vec2i::up(), Coords(self.0 + Vec2i::up())),
            (Vec2i::down(), Coords(self.0 + Vec2i::down())),
            (Vec2i::left(), Coords(self.0 + Vec2i::left())),
            (Vec2i::right(), Coords(self.0 + Vec2i::right())),
        ]
    }
}

pub struct Layer(pub i32);

#[derive(Bundle)]
pub struct BoardObjectBundle {
    pub coords: Coords,
    pub facing: Facing,
    pub transform: Transform,
    pub layer: Layer,
    pub tween: Tween,
}

impl Default for BoardObjectBundle {
    fn default() -> Self {
        BoardObjectBundle {
            coords: Coords(Vec2i::zero()),
            facing: Facing {
                direction: Vec2i::down(),
            },
            transform: Transform::default(),
            layer: Layer(0),
            tween: Tween::default(),
        }
    }
}

pub fn update_position(
    grid: Res<Grid>,
    mut query: Query<(&Transform, &Coords, &Layer, &mut Tween), Changed<Coords>>,
) {
    for (transform, coords, layer, mut tween) in query.iter_mut() {
        let position = grid.map_to_world(coords.0);
        tween.from = transform.translation;
        tween.to = Vec3::new(position.x as f32, position.y as f32, layer.0 as f32);
        tween.start(0.2, TweenMode::Move)
    }
}

pub struct SpatialMap {
    entities: HashMap<Vec2i, Vec<Entity>>,
    coords: HashMap<Entity, Vec2i>,
}

impl SpatialMap {
    pub fn get(&self, position: &Vec2i) -> Option<&Vec<Entity>> {
        self.entities.get(position)
    }
}

impl Default for SpatialMap {
    fn default() -> Self {
        SpatialMap {
            entities: HashMap::default(),
            coords: HashMap::default(),
        }
    }
}

pub fn update_spatial_map(
    mut map: ResMut<SpatialMap>,
    query: Query<(Entity, &Coords), Changed<Coords>>,
    removed: RemovedComponents<Coords>,
) {
    for (entity, coords) in query.iter() {
        // Remove Removed
        let removed_entities = removed.iter();
        for entity in removed_entities {
            if let Some(pos) = map.coords.get(&entity) {
                let pos = *pos;
                let vec = map.entities.get_mut(&pos).unwrap();
                let mut remove: Vec<usize> = Vec::default();
                for i in 0..vec.len() {
                    if vec[i] == entity {
                        remove.push(i);
                    }
                }
                for i in remove {
                    vec.remove(i);
                }
            }
        }
        // Change Changed
        if let Some(prev_pos) = map.coords.get(&entity) {
            let prev_pos = *prev_pos;

            // Remove Previous
            let vec = map.entities.get_mut(&prev_pos).unwrap();
            let mut remove: Vec<usize> = Vec::default();
            for i in 0..vec.len() {
                if vec[i] == entity {
                    remove.push(i);
                }
            }
            for i in remove {
                vec.remove(i);
            }

            // Add Changed
            map.coords.insert(entity, coords.0);
            if let Some(vec) = map.entities.get_mut(&coords.0) {
                vec.push(entity);
            } else {
                map.entities.insert(coords.0, vec![entity]);
            }
        } else {
            // Add New
            map.coords.insert(entity, coords.0);
            map.entities.insert(coords.0, vec![entity]);
        }
    }
}
