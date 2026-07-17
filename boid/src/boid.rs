use macroquad::prelude::*;

const SIZE: f32 = 7.0;

pub struct Boid {
    position: Vec2,
    velocity: Vec2,
}

impl Boid {
    pub fn random(bounds: Vec2, max_speed: f32) -> Self {
        let position = vec2(
            rand::gen_range(0.0, bounds.x),
            rand::gen_range(0.0, bounds.y),
        );
        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let speed = rand::gen_range(max_speed * 0.4, max_speed);
        let velocity = vec2(angle.cos(), angle.sin()) * speed;

        Self { position, velocity }
    }

    pub fn update(&mut self, bounds: Vec2) {
        self.position += self.velocity;
        self.wrap_edges(bounds);
    }

    pub fn draw(&self) {
        let heading = self.velocity.to_angle();
        let nose = self.position + vec2(heading.cos(), heading.sin()) * SIZE;
        let left = self.position + vec2((heading + 2.5).cos(), (heading + 2.5).sin()) * SIZE;
        let right = self.position + vec2((heading - 2.5).cos(), (heading - 2.5).sin()) * SIZE;

        draw_triangle(nose, left, right, WHITE);
    }

    fn wrap_edges(&mut self, bounds: Vec2) {
        if self.position.x < 0.0 {
            self.position.x = bounds.x;
        } else if self.position.x > bounds.x {
            self.position.x = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = bounds.y;
        } else if self.position.y > bounds.y {
            self.position.y = 0.0;
        }
    }
}
