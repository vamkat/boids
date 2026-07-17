use macroquad::prelude::*;

pub const SCREEN_MIN: f32 = 0.0;

const DEFAULT_BOID_COUNT: usize = 80;
const DEFAULT_MAX_SPEED: f32 = 2.5;
const DEFAULT_MIN_INITIAL_SPEED_FACTOR: f32 = 0.4;
const DEFAULT_BOID_SIZE: f32 = 7.0;
const DEFAULT_BOID_WING_ANGLE: f32 = 2.5;

const DEFAULT_BACKGROUND_RED: f32 = 0.02;
const DEFAULT_BACKGROUND_GREEN: f32 = 0.02;
const DEFAULT_BACKGROUND_BLUE: f32 = 0.025;
const DEFAULT_BACKGROUND_ALPHA: f32 = 1.0;

const DEFAULT_BACKGROUND_COLOR: Color = Color {
    r: DEFAULT_BACKGROUND_RED,
    g: DEFAULT_BACKGROUND_GREEN,
    b: DEFAULT_BACKGROUND_BLUE,
    a: DEFAULT_BACKGROUND_ALPHA,
};
const DEFAULT_BOID_COLOR: Color = WHITE;

#[derive(Clone, Copy)]
pub struct Config {
    pub boid_count: usize,
    pub max_speed: f32,
    pub min_initial_speed_factor: f32,
    pub boid_size: f32,
    pub boid_wing_angle: f32,
    pub background_color: Color,
    pub boid_color: Color,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            boid_count: DEFAULT_BOID_COUNT,
            max_speed: DEFAULT_MAX_SPEED,
            min_initial_speed_factor: DEFAULT_MIN_INITIAL_SPEED_FACTOR,
            boid_size: DEFAULT_BOID_SIZE,
            boid_wing_angle: DEFAULT_BOID_WING_ANGLE,
            background_color: DEFAULT_BACKGROUND_COLOR,
            boid_color: DEFAULT_BOID_COLOR,
        }
    }
}
