# Boids

A flocking simulation written in Rust with [macroquad](https://macroquad.rs/).
Each bird ("boid") follows a few simple local rules, and coordinated flocking
emerges from the group without any boid being told what the flock should do.

It implements the [**boids** model](https://www.red3d.com/cwr/boids/) introduced
by Craig Reynolds in 1986, whose three steering rules — separation, alignment,
and cohesion — remain the basis for flocking simulations today.

Everything is tunable live from an on-screen control panel, so the simulation
doubles as a sandbox for exploring how each rule shapes the collective behavior.

## Running

Requires a [Rust toolchain](https://rustup.rs/).

```sh
cargo run --release
```

A window opens with the flock already in motion and the control panel docked on
the right.

## Controls

- **Tab** — show or hide the control panel. While hidden, the flock expands to
  fill the whole window; showing it again reserves the right-hand column and
  eases any stray boids back into view.
- **Sliders and checkboxes** edit the simulation live; changes take effect on the
  next frame.
- **Reset to defaults** — restore every setting and rebuild the flock.
- **Clear trails** — erase the recorded flight paths without changing anything
  else.

## What the boids do

Each boid steers by combining a handful of forces every frame:

- **Separation** — steer away from neighbors that are too close.
- **Alignment** — match the average heading of nearby neighbors.
- **Cohesion** — steer toward the average position of nearby neighbors.
- **Wander** — a small, smoothly changing random turn so motion never becomes
  perfectly uniform.
- **Edge avoidance** — steer away from the window borders before reaching them.
  (Switch to **wrap around** in the panel to have boids reappear on the opposite
  side instead.)

Two details make the flocking feel more lifelike:

- **Field of view.** Each boid only reacts to neighbors inside a vision cone
  centered on its heading, leaving a blind spot behind it. Narrowing the cone
  makes the flock more prone to splitting when it turns — leaders slip into the
  blind spot of the boids behind them. Widening it toward a full turn keeps the
  flock together. This is exposed as the **fov** slider (in degrees).
- **Per-boid variation.** Each boid is created with slightly different speed,
  steering strength, and size, so the flock never moves in lockstep. The amount
  of variation is adjustable under **Variation**.

Boids also leave fading **trails** tracing their recent paths. Trail length,
thickness, opacity, and color are all adjustable, independent of the boid color.

## Configuration

Every tunable value lives in [`src/config.rs`](src/config.rs) as a field of
`Config`, each with a `DEFAULT_*` constant. The on-screen panel edits a live
`Config`; the constants define what **Reset to defaults** restores. To change a
starting value, edit its constant.

## Project layout

| File                | Responsibility                                            |
| ------------------- | --------------------------------------------------------- |
| `src/main.rs`       | Entry point, window setup, and the frame loop.            |
| `src/config.rs`     | The `Config` struct, defaults, and boundary mode.         |
| `src/boid.rs`       | Per-boid state, the steering rules, and drawing.          |
| `src/simulation.rs` | Owns the flock, runs each frame, reconciles flock size.   |
| `src/trail.rs`      | Recorded flight paths, kept separately from the boids.    |
| `src/ui.rs`         | The docked control panel.                                 |
