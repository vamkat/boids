//! On-screen control panel.
//!
//! The panel edits a `Config` in place through Macroquad's immediate-mode UI.
//! Most fields are wired straight to a slider or checkbox; the two that change
//! how much state the simulation holds, `boid_count` and `trail_length`, are
//! edited here but reconciled by `Simulation` on the next tick.

use macroquad::prelude::*;
use macroquad::ui::{Id, Ui, hash, root_ui, widgets};

use crate::config::{BoundaryMode, Config};

/// Key that shows or hides the panel.
const TOGGLE_KEY: KeyCode = KeyCode::Tab;

/// Width of the docked control column.
const PANEL_WIDTH: f32 = 320.0;

/// Gap left around the panel between it and the window edges.
const PANEL_MARGIN: f32 = 10.0;

/// Degrees in a full turn, used to present angle sliders in degrees while the
/// config stores radians.
const DEGREES_PER_TURN: f32 = 360.0;

/// Persistent state for the control panel between frames.
pub struct ControlPanel {
    /// Whether the panel is currently drawn and accepting input.
    visible: bool,
}

/// What the simulation should do in response to this frame's panel input.
///
/// The panel mutates `Config` directly, but a reset also has to rebuild the
/// flock, which only `Simulation` can do.
#[derive(Clone, Copy, Default)]
pub struct ControlOutcome {
    /// The user asked to restore default settings.
    pub reset_requested: bool,

    /// The user asked to erase all recorded trails.
    pub clear_trails_requested: bool,
}

impl ControlPanel {
    /// Creates a control panel that starts visible.
    pub fn new() -> Self {
        Self { visible: true }
    }

    /// Horizontal space the panel reserves at the right of the window.
    ///
    /// The simulation shrinks its play area by this amount so boids never move
    /// behind the panel. A hidden panel reserves nothing and hands the whole
    /// window back to the flock.
    pub fn reserved_width(&self) -> f32 {
        if self.visible {
            PANEL_WIDTH + PANEL_MARGIN * 2.0
        } else {
            0.0
        }
    }

    /// Handles the toggle key and, when visible, draws the panel and applies its
    /// edits to `config`.
    pub fn update(&mut self, config: &mut Config) -> ControlOutcome {
        if is_key_pressed(TOGGLE_KEY) {
            self.visible = !self.visible;
        }

        let mut outcome = ControlOutcome::default();

        if !self.visible {
            return outcome;
        }

        let position = vec2(
            screen_width() - PANEL_WIDTH - PANEL_MARGIN,
            PANEL_MARGIN,
        );
        let size = vec2(PANEL_WIDTH, screen_height() - PANEL_MARGIN * 2.0);

        widgets::Window::new(hash!(), position, size)
            .label("Controls (Tab to hide)")
            .titlebar(true)
            .movable(false)
            .ui(&mut root_ui(), |ui| {
                outcome = panel_body(ui, config);
            });

        outcome
    }
}

/// Lays out every control and returns the actions the simulation must handle.
fn panel_body(ui: &mut Ui, config: &mut Config) -> ControlOutcome {
    let mut outcome = ControlOutcome::default();

    if ui.button(None, "Reset to defaults") {
        outcome.reset_requested = true;
    }

    section(ui, "Flock");
    slide_usize(ui, hash!(), "count", 0.0..400.0, &mut config.boid_count);
    ui.slider(hash!(), "max speed", 0.5..8.0, &mut config.max_speed);
    ui.slider(hash!(), "size", 1.0..20.0, &mut config.boid_size);

    section(ui, "Separation");
    ui.slider(hash!(), "radius", 0.0..120.0, &mut config.separation_radius);
    ui.slider(hash!(), "force", 0.0..2.0, &mut config.separation_force);

    section(ui, "Alignment");
    ui.slider(hash!(), "radius", 0.0..160.0, &mut config.alignment_radius);
    ui.slider(hash!(), "force", 0.0..0.2, &mut config.alignment_force);

    section(ui, "Cohesion");
    ui.slider(hash!(), "radius", 0.0..200.0, &mut config.cohesion_radius);
    ui.slider(hash!(), "force", 0.0..0.2, &mut config.cohesion_force);

    section(ui, "Vision");
    slide_degrees(ui, hash!(), "fov", 10.0..360.0, &mut config.fov_angle);

    section(ui, "Wander");
    ui.slider(hash!(), "force", 0.0..0.1, &mut config.wander_force);
    ui.slider(hash!(), "turn rate", 0.0..0.3, &mut config.wander_turn_rate);

    section(ui, "Edges");
    let mut wrap = config.boundary_mode == BoundaryMode::Wrap;
    ui.checkbox(hash!(), "wrap around", &mut wrap);
    config.boundary_mode = if wrap {
        BoundaryMode::Wrap
    } else {
        BoundaryMode::AvoidEdges
    };
    ui.slider(hash!(), "margin", 0.0..400.0, &mut config.edge_margin);
    ui.slider(hash!(), "avoid force", 0.0..0.3, &mut config.edge_avoidance_force);

    section(ui, "Variation");
    ui.slider(hash!(), "speed", 0.0..1.0, &mut config.speed_variation);
    ui.slider(hash!(), "force", 0.0..1.0, &mut config.force_variation);
    ui.slider(hash!(), "size", 0.0..1.0, &mut config.size_variation);

    section(ui, "Trails");
    ui.checkbox(hash!(), "enabled", &mut config.trails_enabled);
    slide_usize(ui, hash!(), "length", 0.0..200.0, &mut config.trail_length);
    ui.slider(hash!(), "thickness", 0.5..5.0, &mut config.trail_thickness);
    ui.slider(hash!(), "opacity", 0.0..1.0, &mut config.trail_opacity);
    if ui.button(None, "Clear trails") {
        outcome.clear_trails_requested = true;
    }

    section(ui, "Color");
    color_sliders(ui, "boid", &mut config.boid_color);
    color_sliders(ui, "bg", &mut config.background_color);

    outcome
}

/// Draws a labeled separator between groups of controls.
fn section(ui: &mut Ui, title: &str) {
    ui.separator();
    ui.label(None, title);
}

/// Draws a slider bound to a `usize`, rounding the slider's float to the nearest
/// whole value.
fn slide_usize(ui: &mut Ui, id: Id, label: &str, range: std::ops::Range<f32>, value: &mut usize) {
    let mut float_value = *value as f32;
    ui.slider(id, label, range, &mut float_value);
    *value = float_value.round().max(0.0) as usize;
}

/// Draws an angle slider in degrees while storing the value in radians.
fn slide_degrees(ui: &mut Ui, id: Id, label: &str, range: std::ops::Range<f32>, radians: &mut f32) {
    let mut degrees = radians.to_degrees();
    ui.slider(id, label, range, &mut degrees);
    *radians = degrees.to_radians();
    // Guard against a float round-trip drifting the widest setting just under a
    // full turn, which would switch the field-of-view cone back on.
    if degrees >= DEGREES_PER_TURN {
        *radians = std::f32::consts::TAU;
    }
}

/// Draws red, green, and blue sliders for a color, leaving its alpha untouched.
fn color_sliders(ui: &mut Ui, label: &str, color: &mut Color) {
    ui.label(None, label);
    ui.slider(hash!(label, "r"), "r", 0.0..1.0, &mut color.r);
    ui.slider(hash!(label, "g"), "g", 0.0..1.0, &mut color.g);
    ui.slider(hash!(label, "b"), "b", 0.0..1.0, &mut color.b);
}
