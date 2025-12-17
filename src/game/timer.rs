use bevy::prelude::*;

#[derive(Resource)]
pub struct AlgorithmTimers {
    pub a_star_last_ms: f64,
    pub a_star_total_ms: f64,
    pub a_star_max_ms: f64,
    pub hybrid_last_ms: f64,
    pub hybrid_total_ms: f64,
    pub hybrid_max_ms: f64,
}

impl Default for AlgorithmTimers {
    fn default() -> Self {
        Self {
            a_star_last_ms: 0.0,
            a_star_total_ms: 0.0,
            a_star_max_ms: 0.0,
            hybrid_last_ms: 0.0,
            hybrid_total_ms: 0.0,
            hybrid_max_ms: 0.0,
        }
    }
}

impl AlgorithmTimers {
    pub fn reset_totals(&mut self) {
        self.a_star_total_ms = 0.0;
        self.a_star_max_ms = 0.0;
        self.hybrid_total_ms = 0.0;
        self.hybrid_max_ms = 0.0;
    }

    pub fn log_timings(&self) {
        println!(
            "===\nAStar: last: {:.4} ms ; total: {:.4} ms ; max: {:.4} ms\nHybrid: last: {:.4} ms ; total: {:.4} ms ; max: {:.4} ms",
            self.a_star_last_ms,
            self.a_star_total_ms,
            self.a_star_max_ms,
            self.hybrid_last_ms,
            self.hybrid_total_ms,
            self.hybrid_max_ms
        );
    }
}
