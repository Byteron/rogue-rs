use bevy::prelude::*;

use std::{
    hash::Hash,
    ops::{Add, AddAssign, Div},
    time::Duration,
};

pub struct Tween {
    start: Vec3,
    end: Vec3,
    timer: Timer,
}

impl Tween {
    pub fn tween(&mut self, start: Vec3, end: Vec3, duration: Duration) {
        self.start = start;
        self.end = end;
        self.timer.set_duration(duration.as_secs_f32());
        self.timer.reset();
    }

    pub fn tick(&mut self, delta: f32) {
        self.timer.tick(delta);
    }

    pub fn finished(&self) -> bool {
        self.timer.finished()
    }

    pub fn value(&self) -> Vec3 {
        self.start.lerp(self.end, self.timer.percent())
    }
}

impl Default for Tween {
    fn default() -> Self {
        Tween {
            start: Vec3::zero(),
            end: Vec3::zero(),
            timer: Timer::default(),
        }
    }
}

pub fn tween_ticks(time: Res<Time>, mut query: Query<&mut Tween>) {
    for mut tween in query.iter_mut() {
        tween.tick(time.delta_seconds());
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

    pub fn to_vec(&self) -> Vec2 {
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

impl Div<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn div(self, rhs: Coordinates) -> Self::Output {
        Coordinates {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
