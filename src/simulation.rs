use macroquad::prelude::*;

use crate::boid::Boid;
use crate::config::Config;

pub struct Simulation {
    boids: Vec<Boid>,
    config: Config,
}

impl Simulation {
    pub fn new() -> Self {
        let config = Config::default();
        let bounds = screen_bounds();
        let boids = (0..config.boid_count)
            .map(|_| Boid::random(bounds, &config))
            .collect();

        Self { boids, config }
    }

    pub fn tick(&mut self) {
        let bounds = screen_bounds();

        clear_background(self.config.background_color);

        for boid in &mut self.boids {
            boid.update(bounds, &self.config);
            boid.draw(&self.config);
        }
    }
}

fn screen_bounds() -> Vec2 {
    vec2(screen_width(), screen_height())
}
