use crate::game::timer::AlgorithmTimers;
use bevy::prelude::*;

pub fn log_timing_info(timers: Res<AlgorithmTimers>) {
    timers.log_timings();
}

