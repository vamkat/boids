//! Boid state and per-boid behavior.
//!
//! A boid stores only its own movement state. Higher-level orchestration, such
//! as iterating the flock each frame, belongs to `Simulation`.

use macroquad::prelude::*;

use crate::config::{Config, SCREEN_MIN};

/// A single simulated boid.
///
/// The movement model uses the standard position/velocity/acceleration split:
/// forces accumulate into acceleration for one frame, acceleration changes
/// velocity, and velocity changes position.
pub struct Boid {
    /// Current position in screen coordinates.
    position: Vec2,

    /// Current movement vector, measured in pixels per frame.
    velocity: Vec2,

    /// Sum of steering forces waiting to be applied on the next update.
    acceleration: Vec2,
}

impl Boid {
    /// Creates a boid at a random position inside `bounds`.
    ///
    /// The initial velocity is built from a random angle and a random speed so
    /// the flock starts spread across directions instead of moving in lockstep.
    pub fn random(bounds: Vec2, config: &Config) -> Self {
        let position = vec2(
            rand::gen_range(SCREEN_MIN, bounds.x),
            rand::gen_range(SCREEN_MIN, bounds.y),
        );
        let angle = rand::gen_range(SCREEN_MIN, std::f32::consts::TAU);
        let speed = rand::gen_range(
            config.max_speed * config.min_initial_speed_factor,
            config.max_speed,
        );
        // A unit vector from the angle gives direction; multiplying by speed
        // turns it into a velocity vector.
        let velocity = vec2(angle.cos(), angle.sin()) * speed;
        let acceleration = Vec2::ZERO;

        Self {
            position,
            velocity,
            acceleration,
        }
    }

    /// Adds a steering force to be applied during the next `update`.
    ///
    /// Multiple forces can be accumulated before the frame update. Edge
    /// avoidance, separation, alignment, and cohesion will all use this path.
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    /// Applies a steering force away from the window edges.
    ///
    /// The force starts at zero at `edge_margin` and ramps up as the boid gets
    /// closer to the actual edge.
    pub fn avoid_edges(&mut self, bounds: Vec2, config: &Config) {
        if config.edge_margin <= SCREEN_MIN {
            return;
        }

        let mut force = Vec2::ZERO;

        // Horizontal steering: push right near the left edge, and left near the
        // right edge.
        if self.position.x < config.edge_margin {
            force.x +=
                config.edge_avoidance_force * edge_proximity(self.position.x, config.edge_margin);
        } else if self.position.x > bounds.x - config.edge_margin {
            force.x -= config.edge_avoidance_force
                * edge_proximity(bounds.x - self.position.x, config.edge_margin);
        }

        // Vertical steering: push down near the top edge, and up near the
        // bottom edge. In screen coordinates, positive y points downward.
        if self.position.y < config.edge_margin {
            force.y +=
                config.edge_avoidance_force * edge_proximity(self.position.y, config.edge_margin);
        } else if self.position.y > bounds.y - config.edge_margin {
            force.y -= config.edge_avoidance_force
                * edge_proximity(bounds.y - self.position.y, config.edge_margin);
        }

        self.apply_force(force);
    }

    /// Advances this boid by one frame.
    ///
    /// Acceleration is reset after it has affected velocity so each frame starts
    /// with a fresh set of steering forces.
    pub fn update(&mut self, config: &Config) {
        self.velocity += self.acceleration;
        self.limit_speed(config.max_speed);
        self.position += self.velocity;
        self.acceleration = Vec2::ZERO;
    }

    /// Draws this boid as a triangle pointing in its velocity direction.
    pub fn draw(&self, config: &Config) {
        let heading = self.velocity.to_angle();
        let nose = self.position + vec2(heading.cos(), heading.sin()) * config.boid_size;
        let left = self.position
            + vec2(
                (heading + config.boid_wing_angle).cos(),
                (heading + config.boid_wing_angle).sin(),
            ) * config.boid_size;
        let right = self.position
            + vec2(
                (heading - config.boid_wing_angle).cos(),
                (heading - config.boid_wing_angle).sin(),
            ) * config.boid_size;

        draw_triangle(nose, left, right, config.boid_color);
    }

    /// Caps velocity magnitude without changing travel direction.
    fn limit_speed(&mut self, max_speed: f32) {
        if self.velocity.length() > max_speed {
            self.velocity = self.velocity.normalize() * max_speed;
        }
    }
}

/// Returns how deep into the edge margin a boid is.
///
/// The result is `0.0` at the margin boundary and `1.0` at the window edge.
fn edge_proximity(distance_from_edge: f32, margin: f32) -> f32 {
    (margin - distance_from_edge).max(SCREEN_MIN) / margin
}
