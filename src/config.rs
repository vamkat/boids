//! Runtime configuration for the simulation.
//!
//! The default values are currently static, but the `Config` struct is owned by
//! `Simulation` so the same fields can later be edited live by keyboard input or
//! an on-screen control panel.

use macroquad::prelude::*;

/// Minimum coordinate in Macroquad screen space.
///
/// Macroquad's visible window starts at `(0, 0)` in the top-left corner.
pub const SCREEN_MIN: f32 = 0.0;

// Default simulation values. Keep these private so the rest of the code goes
// through `Config`, which is the future live-tuning surface.
const DEFAULT_BOID_COUNT: usize = 80;
const DEFAULT_MAX_SPEED: f32 = 2.5;
const DEFAULT_MIN_INITIAL_SPEED_FACTOR: f32 = 0.4;
const DEFAULT_BOID_SIZE: f32 = 7.0;
const DEFAULT_BOID_WING_ANGLE: f32 = 2.5;
const DEFAULT_EDGE_MARGIN: f32 = 50.0;
const DEFAULT_EDGE_AVOIDANCE_FORCE: f32 = 0.15;
const DEFAULT_SEPARATION_RADIUS: f32 = 50.0;
const DEFAULT_SEPARATION_FORCE: f32 = 0.15;
const DEFAULT_ALIGNMENT_RADIUS: f32 = 55.0;
const DEFAULT_ALIGNMENT_FORCE: f32 = 0.01;
const DEFAULT_COHESION_RADIUS: f32 = 90.0;
const DEFAULT_COHESION_FORCE: f32 = 0.006;
const DEFAULT_BOUNDARY_MODE: BoundaryMode = BoundaryMode::Wrap;

// Color channels are named separately so the default color has no anonymous
// numeric literals in its construction.
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

/// Strategy used when boids approach or cross the window boundary.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum BoundaryMode {
    /// Teleport boids from one side of the screen to the opposite side.
    Wrap,

    /// Steer boids away from edges before they leave the screen.
    AvoidEdges,
}

/// Tunable parameters for a running boid simulation.
///
/// Values that affect behavior or presentation belong here instead of being
/// embedded in `Boid` or `Simulation`.
#[derive(Clone, Copy)]
pub struct Config {
    /// Number of boids created when a simulation starts.
    pub boid_count: usize,

    /// Upper bound for a boid's velocity magnitude, in pixels per frame.
    pub max_speed: f32,

    /// Lower bound for randomized initial speed, expressed as a fraction of
    /// `max_speed`.
    pub min_initial_speed_factor: f32,

    /// Radius-like size used to draw each boid triangle.
    pub boid_size: f32,

    /// Angle offset, in radians, from the boid heading to each rear triangle
    /// point.
    pub boid_wing_angle: f32,

    /// Distance from each window edge where edge avoidance begins.
    pub edge_margin: f32,

    /// Maximum steering force applied when a boid reaches the window edge.
    pub edge_avoidance_force: f32,

    /// Neighbor distance where separation steering begins.
    pub separation_radius: f32,

    /// Maximum steering force used to move away from nearby boids.
    pub separation_force: f32,

    /// Neighbor distance where heading matching begins.
    pub alignment_radius: f32,

    /// Maximum steering force used to match nearby boid headings.
    pub alignment_force: f32,

    /// Neighbor distance where attraction toward local group centers begins.
    pub cohesion_radius: f32,

    /// Maximum steering force used to move toward nearby boid groups.
    pub cohesion_force: f32,

    /// Current boundary behavior.
    pub boundary_mode: BoundaryMode,

    /// Color used to clear the frame before drawing boids.
    pub background_color: Color,

    /// Color used to draw each boid.
    pub boid_color: Color,
}

impl Default for Config {
    /// Builds the current default configuration.
    fn default() -> Self {
        Self {
            boid_count: DEFAULT_BOID_COUNT,
            max_speed: DEFAULT_MAX_SPEED,
            min_initial_speed_factor: DEFAULT_MIN_INITIAL_SPEED_FACTOR,
            boid_size: DEFAULT_BOID_SIZE,
            boid_wing_angle: DEFAULT_BOID_WING_ANGLE,
            edge_margin: DEFAULT_EDGE_MARGIN,
            edge_avoidance_force: DEFAULT_EDGE_AVOIDANCE_FORCE,
            separation_radius: DEFAULT_SEPARATION_RADIUS,
            separation_force: DEFAULT_SEPARATION_FORCE,
            alignment_radius: DEFAULT_ALIGNMENT_RADIUS,
            alignment_force: DEFAULT_ALIGNMENT_FORCE,
            cohesion_radius: DEFAULT_COHESION_RADIUS,
            cohesion_force: DEFAULT_COHESION_FORCE,
            boundary_mode: DEFAULT_BOUNDARY_MODE,
            background_color: DEFAULT_BACKGROUND_COLOR,
            boid_color: DEFAULT_BOID_COLOR,
        }
    }
}
