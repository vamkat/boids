//! Program entry point.
//!
//! The main loop stays intentionally small: simulation state and behavior live in
//! the `simulation` and `boid` modules.

mod boid;
mod config;
mod simulation;
mod trail;
mod ui;

use macroquad::prelude::*;

use simulation::Simulation;

/// Initial window size, in pixels.
///
/// The window is deliberately wide: the control panel occupies a reserved column
/// on the right, so the boids need the extra room to keep a large play area.
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 800;

/// Window configuration applied before the simulation starts.
fn window_conf() -> Conf {
    Conf {
        window_title: "Boids".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::new();

    loop {
        simulation.tick();
        macroquad::prelude::next_frame().await;
    }
}
