//! Simulation orchestration.
//!
//! This module owns the flock and shared configuration, leaving individual boid
//! mechanics in `boid.rs`.

use macroquad::prelude::*;

use crate::boid::Boid;
use crate::config::Config;

/// Running simulation state.
pub struct Simulation {
    /// All boids currently participating in the flock.
    boids: Vec<Boid>,

    /// Runtime-adjustable simulation settings.
    config: Config,
}

impl Simulation {
    /// Creates a new simulation using the default config.
    pub fn new() -> Self {
        let config = Config::default();
        let bounds = screen_bounds();
        let boids = (0..config.boid_count)
            .map(|_| Boid::random(bounds, &config))
            .collect();

        Self { boids, config }
    }

    /// Runs one frame of simulation and rendering.
    pub fn tick(&mut self) {
        let bounds = screen_bounds();
        let flock_snapshot = self.boids.clone();

        clear_background(self.config.background_color);

        for boid in &mut self.boids {
            boid.separate(&flock_snapshot, &self.config);
            boid.align(&flock_snapshot, &self.config);
            boid.avoid_edges(bounds, &self.config);
            boid.update(&self.config);
            boid.draw(&self.config);
        }
    }
}

/// Current drawable window size.
fn screen_bounds() -> Vec2 {
    vec2(screen_width(), screen_height())
}
