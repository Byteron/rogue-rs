use bevy::math::{Vec2, Vec3};
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

    pub fn as_f32(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

impl Default for Vec3i {
    fn default() -> Self {
        Vec3i { x: 0, y: 0, z: 0 }
    }
}

impl Mul<Vec3i> for Vec3i {
    type Output = Vec3i;

    fn mul(self, rhs: Vec3i) -> Self::Output {
        Vec3i {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
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

    pub fn up() -> Self {
        Vec2i { x: 0, y: 1 }
    }

    pub fn down() -> Self {
        Vec2i { x: 0, y: -1 }
    }

    pub fn left() -> Self {
        Vec2i { x: -1, y: 0 }
    }

    pub fn right() -> Self {
        Vec2i { x: 1, y: 0 }
    }

    pub fn extend(&self, z: i32) -> Vec3i {
        Vec3i::new(self.x, self.y, z)
    }

    pub fn as_f32(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
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

impl Mul<i32> for Vec2i {
    type Output = Vec2i;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2i {
            x: self.x * rhs,
            y: self.y * rhs,
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
