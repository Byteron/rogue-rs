use bevy::prelude::*;

use std::{
    hash::Hash,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

pub struct Grid {
    pub cell_size: Vec2,
}

impl Grid {
    pub fn map_to_world(&self, coords: Coordinates) -> Vec3 {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinates { x, y }
    }

    pub fn zero() -> Self {
        Coordinates { x: 0, y: 0 }
    }

    pub fn as_f32(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn get_neighbors(&self) -> [Coordinates; 4] {
        [
            *self + Coordinates::new(0, -1),
            *self + Coordinates::new(-1, 0),
            *self + Coordinates::new(1, 0),
            *self + Coordinates::new(0, 1),
        ]
    }
}

impl Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Coordinates> for Coordinates {
    fn add_assign(&mut self, rhs: Coordinates) {
        *self = *self + rhs;
    }
}

impl Sub<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Coordinates) -> Self::Output {
        Coordinates {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: Coordinates) -> Self::Output {
        Coordinates {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn div(self, rhs: Coordinates) -> Self::Output {
        Coordinates {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
