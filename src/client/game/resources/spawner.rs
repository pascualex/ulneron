use bevy::prelude::*;

pub struct Spawner {
    pub timer: Timer,
}

impl Spawner {
    pub fn from_hertz(frequency: f32) -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / frequency, true),
        }
    }
}