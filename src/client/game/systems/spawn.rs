use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    client::game::{
        components::{Agent, Enemy, Health, Position, Size, Stats, Velocity},
        resources::{Random, Spawner, Ticks},
    },
    TICK_STEP,
};

pub fn spawn(
    mut spawner: ResMut<Spawner>,
    mut random: ResMut<Random>,
    ticks: Res<Ticks>,
    mut commands: Commands,
) {
    if ticks.current.is_none() {
        return;
    }

    spawner.timer.tick(Duration::from_secs_f32(TICK_STEP));
    let spawn_count = spawner.timer.times_finished_this_tick() * spawner.multiplier;
    for _ in 0..spawn_count {
        let direction = match (random.gen(), random.gen()) {
            (true, true) => Vec2::Y,
            (true, false) => Vec2::X,
            (false, true) => -Vec2::Y,
            (false, false) => -Vec2::X,
        };
        let offset = Vec2::new(random.gen_range(-1.0..=1.0), random.gen_range(-1.0..=1.0));
        let position = direction * 20.0 + offset * 5.0;
        commands.spawn((
            Position::new(position),
            Velocity::new(),
            Size::new(0.25),
            Stats::new(1.0),
            Agent::new(),
            Health::new(10),
            Enemy,
        ));
    }
}
