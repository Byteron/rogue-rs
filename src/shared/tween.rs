use bevy::prelude::*;
use std::time::Duration;

pub enum TweenMode {
    Move,
    Attack,
}

pub struct Tween {
    pub from: Vec3,
    pub to: Vec3,
    timer: Timer,
    mode: TweenMode,
}

impl Tween {
    pub fn new(translation: Vec3) -> Self {
        Tween {
            from: translation,
            to: translation,
            ..Default::default()
        }
    }

    pub fn value(&self) -> Vec3 {
        let time: f32;

        match self.mode {
            TweenMode::Move => {
                time = self.timer.percent();
            }
            TweenMode::Attack => {
                time = (1.0 - (self.timer.percent() * 2.0 - 1.0).abs()) / 2.0;
            }
        }

        self.from.lerp(self.to, time)
    }

    pub fn tick(&mut self, delta: f32) {
        self.timer.tick(delta);
    }

    pub fn start(&mut self, duration: f32, mode: TweenMode) {
        self.mode = mode;
        self.timer.set_duration(duration);
        self.timer.reset();
    }

    pub fn finished(&self) -> bool {
        self.timer.finished()
    }
}

impl Default for Tween {
    fn default() -> Self {
        Tween {
            from: Vec3::zero(),
            to: Vec3::zero(),
            timer: Timer::new(Duration::from_secs_f32(0.15), false),
            mode: TweenMode::Move,
        }
    }
}

pub struct TweenPlugin;
impl Plugin for TweenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(stage::PRE_UPDATE, tween.system());
    }
}

pub fn tween(time: Res<Time>, mut query: Query<(&mut Transform, &mut Tween)>) {
    for (mut transform, mut tween) in query.iter_mut() {
        if !tween.finished() {
            tween.tick(time.delta_seconds());
            transform.translation = tween.value();
        }
    }
}
