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
pub const BASE_MULTIPLIER: f32 = 1.0;

/// Fraction used to split a whole value into two equal parts.
pub const HALF: f32 = 0.5;

// Default simulation values. Keep these private so the rest of the code goes
// through `Config`, which is the future live-tuning surface.
const DEFAULT_BOID_COUNT: usize = 80;
const DEFAULT_MAX_SPEED: f32 = 2.5;
const DEFAULT_MIN_INITIAL_SPEED_FACTOR: f32 = 0.4;
const DEFAULT_BOID_SIZE: f32 = 7.0;
const DEFAULT_BOID_WING_ANGLE: f32 = 2.5;
const DEFAULT_EDGE_MARGIN: f32 = 200.0;
const DEFAULT_EDGE_AVOIDANCE_FORCE: f32 = 0.05;
const DEFAULT_SEPARATION_RADIUS: f32 = 30.0;
const DEFAULT_SEPARATION_FORCE: f32 = 0.85;
const DEFAULT_ALIGNMENT_RADIUS: f32 = 55.0;
const DEFAULT_ALIGNMENT_FORCE: f32 = 0.02;
const DEFAULT_COHESION_RADIUS: f32 = 70.0;
const DEFAULT_COHESION_FORCE: f32 = 0.03;
// Three quarters of a full turn, leaving a 90 degree blind spot behind each
// boid.
const DEFAULT_FOV_ANGLE: f32 = std::f32::consts::TAU * 0.75;
const DEFAULT_WANDER_FORCE: f32 = 0.015;
const DEFAULT_WANDER_TURN_RATE: f32 = 0.05;
// These two lengths only matter as a ratio. A radius half the distance bounds
// wander steering to 30 degrees off the heading.
const DEFAULT_WANDER_DISTANCE: f32 = 12.0;
const DEFAULT_WANDER_RADIUS: f32 = 6.0;
const DEFAULT_SPEED_VARIATION: f32 = 0.52;
const DEFAULT_FORCE_VARIATION: f32 = 0.30;
const DEFAULT_SIZE_VARIATION: f32 = 0.10;
const DEFAULT_BOUNDARY_MODE: BoundaryMode = BoundaryMode::AvoidEdges;
const DEFAULT_TRAILS_ENABLED: bool = true;
const DEFAULT_TRAIL_LENGTH: usize = 45;
const DEFAULT_TRAIL_THICKNESS: f32 = 1.0;
const DEFAULT_TRAIL_OPACITY: f32 = 0.35;

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
#[derive(Clone, Copy, PartialEq, Eq)]
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

    /// Total width, in radians, of the vision cone centered on a boid's
    /// heading.
    ///
    /// Neighbors outside this cone are ignored by separation, alignment, and
    /// cohesion. A value of `TAU` or greater disables the restriction, and a
    /// boid moving too slowly to have a meaningful heading also sees in every
    /// direction.
    pub fov_angle: f32,

    /// Continuous steering force that prevents perfectly uniform motion.
    pub wander_force: f32,

    /// Maximum random angle change, in radians, applied per frame to a boid's
    /// position on its wander ring.
    pub wander_turn_rate: f32,

    /// How far ahead of a boid, along its heading, the wander ring is centered.
    ///
    /// Raising this relative to `wander_radius` narrows wander steering toward
    /// the heading.
    pub wander_distance: f32,

    /// Radius of the wander ring.
    ///
    /// Together with `wander_distance` this bounds how far wander can deflect a
    /// boid from its heading, to `asin(wander_radius / wander_distance)`. Values
    /// at or above `wander_distance` remove that bound and let wander oppose the
    /// boid's own travel.
    pub wander_radius: f32,

    /// Per-boid speed variation around `1.0`.
    ///
    /// A value of `0.12` gives each boid a speed multiplier in the range
    /// `0.88..1.12` when it is created.
    pub speed_variation: f32,

    /// Per-boid steering-force variation around `1.0`.
    pub force_variation: f32,

    /// Per-boid rendered-size variation around `1.0`.
    pub size_variation: f32,

    /// Current boundary behavior.
    pub boundary_mode: BoundaryMode,

    /// Whether boids draw their recent flight path.
    pub trails_enabled: bool,

    /// Number of past positions kept per boid for its trail.
    pub trail_length: usize,

    /// Line width used to draw trail segments.
    pub trail_thickness: f32,

    /// Overall opacity of trails, scaling the per-segment age fade.
    pub trail_opacity: f32,

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
            fov_angle: DEFAULT_FOV_ANGLE,
            wander_force: DEFAULT_WANDER_FORCE,
            wander_turn_rate: DEFAULT_WANDER_TURN_RATE,
            wander_distance: DEFAULT_WANDER_DISTANCE,
            wander_radius: DEFAULT_WANDER_RADIUS,
            speed_variation: DEFAULT_SPEED_VARIATION,
            force_variation: DEFAULT_FORCE_VARIATION,
            size_variation: DEFAULT_SIZE_VARIATION,
            boundary_mode: DEFAULT_BOUNDARY_MODE,
            trails_enabled: DEFAULT_TRAILS_ENABLED,
            trail_length: DEFAULT_TRAIL_LENGTH,
            trail_thickness: DEFAULT_TRAIL_THICKNESS,
            trail_opacity: DEFAULT_TRAIL_OPACITY,
            background_color: DEFAULT_BACKGROUND_COLOR,
            boid_color: DEFAULT_BOID_COLOR,
        }
    }
}
