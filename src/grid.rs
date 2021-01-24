use bevy::prelude::*;

use std::{
    hash::Hash,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3i {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3i { x, y, z }
    }

    pub fn zero() -> Vec3i {
        Vec3i::default()
    }

    pub fn reduce(&self) -> Vec2i {
        Vec2i {
            x: self.x,
            y: self.y,
        }
    }
}

impl Default for Vec3i {
    fn default() -> Self {
        Vec3i { x: 0, y: 0, z: 0 }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2i { x, y }
    }

    pub fn zero() -> Self {
        Vec2i { x: 0, y: 0 }
    }

    pub fn extend(&self, z: i32) -> Vec3i {
        Vec3i::new(self.x, self.y, z)
    }

    pub fn as_f32(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn get_neighbors(&self) -> [Vec2i; 4] {
        [
            *self + Vec2i::new(0, -1),
            *self + Vec2i::new(-1, 0),
            *self + Vec2i::new(1, 0),
            *self + Vec2i::new(0, 1),
        ]
    }
}

impl Add<Vec2i> for Vec2i {
    type Output = Vec2i;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vec2i> for Vec2i {
    fn add_assign(&mut self, rhs: Vec2i) {
        *self = *self + rhs;
    }
}

impl Sub<Vec2i> for Vec2i {
    type Output = Vec2i;

    fn sub(self, rhs: Vec2i) -> Self::Output {
        Vec2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Vec2i> for Vec2i {
    type Output = Vec2i;

    fn mul(self, rhs: Vec2i) -> Self::Output {
        Vec2i {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div<Vec2i> for Vec2i {
    type Output = Vec2i;

    fn div(self, rhs: Vec2i) -> Self::Output {
        Vec2i {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

pub struct Grid {
    pub cell_size: Vec2,
}

impl Grid {
    pub fn map_to_world(&self, coords: Vec2i) -> Vec3 {
        (coords.as_f32() * self.cell_size).extend(0.0)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            cell_size: Vec2::new(64.0, 64.0),
        }
    }
}

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Grid::default());
    }
}
