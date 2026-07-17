mod boid;
mod config;
mod simulation;

use simulation::Simulation;

#[macroquad::main("Boids")]
async fn main() {
    let mut simulation = Simulation::new();

    loop {
        simulation.tick();
        macroquad::prelude::next_frame().await;
    }
}
