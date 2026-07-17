//! Recorded flight paths.
//!
//! A trail is kept outside its `Boid` on purpose. `Boid` is `Copy` and the whole
//! flock is cloned once per frame to give the rules a stable snapshot, so growing
//! a boid with a position history would make that snapshot expensive. `Simulation`
//! instead owns a `Trail` per boid at the same index.

use std::collections::VecDeque;

use macroquad::prelude::*;

use crate::config::{BASE_MULTIPLIER, Config, SCREEN_MIN};

/// Fewest points that can describe a drawable path.
///
/// One point is a position with nothing to connect it to.
const MIN_DRAWABLE_POINTS: usize = 2;

/// Multiple of a boid's per-frame travel distance above which a trail segment is
/// treated as a teleport rather than movement.
///
/// Wrapping moves a boid across the window between two recorded points. Without
/// this test the resulting segment would be drawn as a line straight back across
/// the screen. The bound is generous because a boid at full speed should never
/// be mistaken for one that wrapped.
const TELEPORT_SEGMENT_FACTOR: f32 = 4.0;

/// The recent positions of a single boid.
pub struct Trail {
    /// Past positions, oldest first.
    ///
    /// A deque keeps both ends cheap: new positions are pushed to the back and
    /// expired ones leave from the front.
    points: VecDeque<Vec2>,
}

impl Trail {
    /// Creates a trail with no recorded history.
    pub fn new() -> Self {
        Self {
            points: VecDeque::new(),
        }
    }

    /// Appends `position` and drops points beyond the configured length.
    ///
    /// Shortening `trail_length` at runtime can leave more points stored than are
    /// wanted, so the excess is discarded here rather than only one point per
    /// frame.
    pub fn record(&mut self, position: Vec2, config: &Config) {
        self.points.push_back(position);

        while self.points.len() > config.trail_length {
            self.points.pop_front();
        }
    }

    /// Forgets all recorded history.
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Draws the path as a line fading from transparent at the tail to the boid
    /// color at the head.
    pub fn draw(&self, config: &Config) {
        if self.points.len() < MIN_DRAWABLE_POINTS {
            return;
        }

        let teleport_length = teleport_length(config);
        // The oldest point starts a segment but never ends one, so the count of
        // segments is one less than the count of points.
        let segments = (self.points.len() - 1) as f32;

        for (index, (&from, &to)) in self.points.iter().zip(self.points.iter().skip(1)).enumerate() {
            if (to - from).length() > teleport_length {
                continue;
            }

            // Segment 0 is the oldest and is drawn nearly transparent; the last
            // one meets the boid at full opacity.
            let age_fraction = (index + 1) as f32 / segments;
            let color = Color {
                a: config.trail_color.a * age_fraction * config.trail_opacity,
                ..config.trail_color
            };

            draw_line(from.x, from.y, to.x, to.y, config.trail_thickness, color);
        }
    }
}

/// Shortest distance between two recorded points that can only be explained by a
/// boid wrapping across the window.
fn teleport_length(config: &Config) -> f32 {
    let fastest_boid_speed = config.max_speed * (BASE_MULTIPLIER + config.speed_variation.max(SCREEN_MIN));

    fastest_boid_speed * TELEPORT_SEGMENT_FACTOR
}
