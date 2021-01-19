use bevy::{math, prelude::*};

use std::{hash::Hash, ops::{Add, AddAssign, Div, Mul, Sub}, time::Duration};

use crate::{main, player::Player};

pub struct Grid {
    pub cell_size: Vec2,
}

impl Grid {
    pub fn map_to_world(&self, coords: Coordinates) -> Vec3 {
        (coords.to_vec() * self.cell_size).extend(0.0)
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

pub enum StepperMode {
    Move,
    Attack
}

pub struct Stepper {
    pub from: Vec3,
    pub to: Vec3,
    timer: Timer,
    mode: StepperMode
}

impl Stepper {
    pub fn new(translation: Vec3) -> Self {
        Stepper {
            from: translation,
            to: translation,
            ..Default::default()
        }
    }

    pub fn value(&self) -> Vec3 {
        let time: f32;

        match self.mode {
            StepperMode::Move => {
                time = self.timer.percent();
            }
            StepperMode::Attack => {
                time = (1.0 - (self.timer.percent() * 2.0 - 1.0).abs()) / 2.0;
            }
        }

        self.from.lerp(self.to, time)
    }

    pub fn tick(&mut self, delta: f32) {
        self.timer.tick(delta);
    }

    pub fn start(&mut self, duration: f32, mode: StepperMode) {
        self.mode = mode;
        self.timer.set_duration(duration);
        self.timer.reset();
    }

    pub fn finished(&self) -> bool {
        self.timer.finished()
    }
}

impl Default for Stepper {
    fn default() -> Self {
        Stepper {
            from: Vec3::zero(),
            to: Vec3::zero(),
            timer: Timer::new(Duration::from_secs_f32(0.15), false),
            mode: StepperMode::Move
        }
    }
}

pub fn step(mut query: Query<(&mut Transform, &Stepper), With<Player>>) {
    for (mut transform, stepper) in query.iter_mut() {
        if stepper.finished() {
            continue;
        }
        transform.translation = stepper.value();
    }
}
