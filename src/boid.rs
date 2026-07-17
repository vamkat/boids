use macroquad::prelude::*;

use crate::config::{Config, SCREEN_MIN};

pub struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Boid {
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
        let velocity = vec2(angle.cos(), angle.sin()) * speed;
        let acceleration = Vec2::ZERO;

        Self {
            position,
            velocity,
            acceleration,
        }
    }

    #[allow(dead_code)]
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update(&mut self, bounds: Vec2, config: &Config) {
        self.velocity += self.acceleration;
        self.limit_speed(config.max_speed);
        self.position += self.velocity;
        self.acceleration = Vec2::ZERO;

        self.wrap_edges(bounds);
    }

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

    fn wrap_edges(&mut self, bounds: Vec2) {
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

    fn limit_speed(&mut self, max_speed: f32) {
        if self.velocity.length() > max_speed {
            self.velocity = self.velocity.normalize() * max_speed;
        }
    }
}
