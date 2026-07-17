//! Simulation orchestration.
//!
//! This module owns the flock and shared configuration, leaving individual boid
//! mechanics in `boid.rs`.

use macroquad::prelude::*;

use crate::boid::Boid;
use crate::config::{BoundaryMode, Config};
use crate::trail::Trail;
use crate::ui::{ControlOutcome, ControlPanel};

/// Running simulation state.
pub struct Simulation {
    /// All boids currently participating in the flock.
    boids: Vec<Boid>,

    /// Recorded flight paths, held at the same index as their boid.
    trails: Vec<Trail>,

    /// Runtime-adjustable simulation settings.
    config: Config,

    /// On-screen controls that edit `config` live.
    controls: ControlPanel,
}

impl Simulation {
    /// Creates a new simulation using the default config.
    pub fn new() -> Self {
        let config = Config::default();
        let controls = ControlPanel::new();
        let bounds = simulation_bounds(&controls);
        let boids = (0..config.boid_count)
            .map(|_| Boid::random(bounds, &config))
            .collect();
        let trails = (0..config.boid_count).map(|_| Trail::new()).collect();

        Self {
            boids,
            trails,
            config,
            controls,
        }
    }

    /// Runs one frame of simulation and rendering.
    pub fn tick(&mut self) {
        let outcome = self.controls.update(&mut self.config);
        // Panel visibility can change how much room the flock has, so the play
        // area is measured after the panel has handled this frame's input.
        let bounds = simulation_bounds(&self.controls);
        self.apply_outcome(outcome, bounds);
        self.reconcile_flock_size(bounds);

        let flock_snapshot = self.boids.clone();

        clear_background(self.config.background_color);

        for trail in &self.trails {
            if self.config.trails_enabled {
                trail.draw(&self.config);
            }
        }

        for (boid, trail) in self.boids.iter_mut().zip(self.trails.iter_mut()) {
            boid.separate(&flock_snapshot, &self.config);
            boid.align(&flock_snapshot, &self.config);
            boid.cohere(&flock_snapshot, &self.config);
            boid.wander(&self.config);

            if let BoundaryMode::AvoidEdges = self.config.boundary_mode {
                boid.avoid_edges(bounds, &self.config);
            }

            boid.update(&self.config);

            if let BoundaryMode::Wrap = self.config.boundary_mode {
                boid.wrap_edges(bounds);
            }

            trail.record(boid.position(), &self.config);
            boid.draw(&self.config);
        }
    }

    /// Carries out the reset and clear actions requested by the control panel.
    fn apply_outcome(&mut self, outcome: ControlOutcome, bounds: Vec2) {
        if outcome.reset_requested {
            self.config = Config::default();
            self.boids.clear();
            self.trails.clear();
            self.reconcile_flock_size(bounds);
        }

        if outcome.clear_trails_requested {
            for trail in &mut self.trails {
                trail.clear();
            }
        }
    }

    /// Grows or shrinks the flock so its size matches `config.boid_count`.
    ///
    /// Trails are kept index-aligned with boids, so both vectors are resized
    /// together. New boids spawn at random positions with empty trails.
    fn reconcile_flock_size(&mut self, bounds: Vec2) {
        let target = self.config.boid_count;

        while self.boids.len() > target {
            self.boids.pop();
            self.trails.pop();
        }

        while self.boids.len() < target {
            self.boids.push(Boid::random(bounds, &self.config));
            self.trails.push(Trail::new());
        }
    }
}

/// Drawable area available to the flock.
///
/// This is the window minus the column reserved by the control panel, so boids
/// stay clear of the controls.
fn simulation_bounds(controls: &ControlPanel) -> Vec2 {
    vec2(
        screen_width() - controls.reserved_width(),
        screen_height(),
    )
}
