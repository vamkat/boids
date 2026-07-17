use macroquad::prelude::*;

use crate::boid::Boid;

const BACKGROUND: Color = Color::new(0.02, 0.02, 0.025, 1.0);
const BOID_COUNT: usize = 80;
const MAX_SPEED: f32 = 2.5;

pub struct Simulation {
    boids: Vec<Boid>,
}

impl Simulation {
    pub fn new() -> Self {
        let bounds = screen_bounds();
        let boids = (0..BOID_COUNT)
            .map(|_| Boid::random(bounds, MAX_SPEED))
            .collect();

        Self { boids }
    }

    pub fn tick(&mut self) {
        let bounds = screen_bounds();

        clear_background(BACKGROUND);

        for boid in &mut self.boids {
            boid.update(bounds);
            boid.draw();
        }
    }
}

fn screen_bounds() -> Vec2 {
    vec2(screen_width(), screen_height())
}
