use bevy::prelude::*;

use super::{grid::{Coords, Grid}, math::Vec2i, tween::{Tween, TweenMode}};

pub struct Moveable {
    from: Vec2i,
    pub to: Vec2i,
}

impl Default for Moveable {
    fn default() -> Self {
        Moveable {
            from: Vec2i::zero(),
            to: Vec2i::zero(),
        }
    }
}

#[derive(Bundle)]
pub struct MoveableBundle {
    moveable: Moveable,
    coords: Coords,
    tween: Tween,
}

impl MoveableBundle {
    pub fn new(x: i32, y: i32) -> Self {
        let pos = Vec2i::new(x, y);

        MoveableBundle {
            moveable: Moveable {
                from: pos,
                to: pos,
            },
            coords: Coords(pos),
            tween: Tween::default(),
        }
    }
}

impl Default for MoveableBundle {
    fn default() -> Self {
        MoveableBundle {
            moveable: Moveable::default(),
            coords: Coords(Vec2i::zero()),
            tween: Tween::default(),
        }
    }
}

pub struct MoveablePlugin;
impl Plugin for MoveablePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update.system());
    }
}

pub fn update(
    grid: Res<Grid>,
    mut query: Query<(&mut Moveable, &mut Tween, &mut Coords), Changed<Moveable>>
) {
    for (mut moveable, mut tween, mut coords) in query.iter_mut() {
        if moveable.from == moveable.to {
            continue;
        }
        
        tween.from = grid.map_to_world(moveable.from.extend(0)).as_f32();
        tween.to = grid.map_to_world(moveable.to.extend(0)).as_f32();
        tween.start(0.25, TweenMode::Move);
        
        moveable.from = moveable.to;
        coords.0 = moveable.to;
    }
}