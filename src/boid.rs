use macroquad::prelude::*;

use crate::config::{Config, SCREEN_MIN};

pub struct Boid {
    position: Vec2,
    velocity: Vec2,
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

        Self { position, velocity }
    }

    pub fn update(&mut self, bounds: Vec2) {
        self.position += self.velocity;
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
}
