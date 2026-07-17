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
#[derive(Clone, Copy)]
pub struct Boid {
    /// Current position in screen coordinates.
    position: Vec2,

    /// Current movement vector, measured in pixels per frame.
    velocity: Vec2,

    /// Sum of steering forces waiting to be applied on the next update.
    acceleration: Vec2,

    /// Internal direction used for smooth random steering.
    wander_angle: f32,
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
        let wander_angle = rand::gen_range(SCREEN_MIN, std::f32::consts::TAU);

        Self {
            position,
            velocity,
            acceleration,
            wander_angle,
        }
    }

    /// Adds a steering force to be applied during the next `update`.
    ///
    /// Multiple forces can be accumulated before the frame update. Edge
    /// avoidance, separation, alignment, and cohesion will all use this path.
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    /// Applies the separation rule against nearby boids.
    ///
    /// Separation pushes this boid away from neighbors within
    /// `separation_radius`. Closer neighbors contribute a stronger push.
    pub fn separate(&mut self, flock: &[Boid], config: &Config) {
        if config.separation_radius <= SCREEN_MIN {
            return;
        }

        let mut steering = Vec2::ZERO;
        let mut neighbors = usize::default();

        for other in flock {
            let offset = self.position - other.position;
            let distance = offset.length();

            if distance > f32::EPSILON && distance < config.separation_radius {
                steering += offset.normalize() / distance;
                neighbors += 1;
            }
        }

        if neighbors > usize::default() {
            steering /= neighbors as f32;
            self.apply_force(limit_vector(steering, config.separation_force));
        }
    }

    /// Applies the alignment rule against nearby boids.
    ///
    /// Alignment steers this boid toward the average velocity of neighbors
    /// within `alignment_radius`. This changes heading, not position directly.
    pub fn align(&mut self, flock: &[Boid], config: &Config) {
        if config.alignment_radius <= SCREEN_MIN {
            return;
        }

        let mut average_velocity = Vec2::ZERO;
        let mut neighbors = usize::default();

        for other in flock {
            let distance = self.position.distance(other.position);

            if distance > f32::EPSILON && distance < config.alignment_radius {
                average_velocity += other.velocity;
                neighbors += 1;
            }
        }

        if neighbors > usize::default() {
            average_velocity /= neighbors as f32;

            if average_velocity.length() <= f32::EPSILON {
                return;
            }

            // Steering is the change needed to move from the current velocity
            // toward the neighbors' average heading. The desired velocity uses
            // `max_speed` so mixed neighbor speeds do not accidentally slow the
            // boid down.
            let desired_velocity = average_velocity.normalize() * config.max_speed;
            let steering = desired_velocity - self.velocity;
            self.apply_force(limit_vector(steering, config.alignment_force));
        }
    }

    /// Applies the cohesion rule against nearby boids.
    ///
    /// Cohesion steers this boid toward the average position of neighbors
    /// within `cohesion_radius`. This is the rule that turns nearby individuals
    /// into visible groups.
    pub fn cohere(&mut self, flock: &[Boid], config: &Config) {
        if config.cohesion_radius <= SCREEN_MIN {
            return;
        }

        let mut average_position = Vec2::ZERO;
        let mut neighbors = usize::default();

        for other in flock {
            let distance = self.position.distance(other.position);

            if distance > f32::EPSILON && distance < config.cohesion_radius {
                average_position += other.position;
                neighbors += 1;
            }
        }

        if neighbors > usize::default() {
            average_position /= neighbors as f32;

            let desired_direction = average_position - self.position;

            if desired_direction.length() <= f32::EPSILON {
                return;
            }

            let desired_velocity = desired_direction.normalize() * config.max_speed;
            let steering = desired_velocity - self.velocity;
            self.apply_force(limit_vector(steering, config.cohesion_force));
        }
    }

    /// Applies a small, smoothly changing random steering force.
    ///
    /// Wander keeps the flock from settling into a perfectly regular spacing
    /// and heading. The force direction changes gradually because it is based on
    /// an angle stored by the boid instead of a fresh random vector each frame.
    pub fn wander(&mut self, config: &Config) {
        if config.wander_force <= SCREEN_MIN || config.wander_turn_rate <= SCREEN_MIN {
            return;
        }

        self.wander_angle += rand::gen_range(-config.wander_turn_rate, config.wander_turn_rate);

        let force = vec2(self.wander_angle.cos(), self.wander_angle.sin()) * config.wander_force;
        self.apply_force(force);
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

    /// Wraps this boid to the opposite side after it crosses a screen edge.
    pub fn wrap_edges(&mut self, bounds: Vec2) {
        if self.position.x < SCREEN_MIN {
            self.position.x = bounds.x;
        } else if self.position.x > bounds.x {
            self.position.x = SCREEN_MIN;
        }

        if self.position.y < SCREEN_MIN {
            self.position.y = bounds.y;
        } else if self.position.y > bounds.y {
            self.position.y = SCREEN_MIN;
        }
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
        self.velocity = limit_vector(self.velocity, max_speed);
    }
}

/// Returns how deep into the edge margin a boid is.
///
/// The result is `0.0` at the margin boundary and `1.0` at the window edge.
fn edge_proximity(distance_from_edge: f32, margin: f32) -> f32 {
    (margin - distance_from_edge).max(SCREEN_MIN) / margin
}

/// Caps a vector's magnitude while preserving its direction.
fn limit_vector(vector: Vec2, max_length: f32) -> Vec2 {
    if vector.length() > max_length {
        vector.normalize() * max_length
    } else {
        vector
    }
}
