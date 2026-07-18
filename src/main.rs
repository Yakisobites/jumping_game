mod config;
mod entities;
mod physics;
mod render;

use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::config::*;
use crate::entities::{Player, spawn_stage};
use crate::physics::PhysicsWorld;
use crate::render::{Camera, draw_player, draw_stage};

// Configure game state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Title,
    Playing,
    GameOver,
}

// reset the initial state of the physics world and the camera
fn reset_game(physics: &mut PhysicsWorld, camera: &mut Camera) -> Player {
    *physics = PhysicsWorld::new();
    *camera = Camera::new();

    spawn_stage(physics);
    Player::spawn(physics)
}

fn consume_physics_steps(accumulator: &mut f32, frame_time: f32) -> usize {
    *accumulator += frame_time.min(MAX_FRAME_TIME);

    let mut steps = 0;
    while *accumulator >= TIME_DELTA && steps < MAX_PHYSICS_STEPS_PER_FRAME {
        *accumulator -= TIME_DELTA;
        steps += 1;
    }

    if steps == MAX_PHYSICS_STEPS_PER_FRAME {
        *accumulator = 0.0;
    }

    steps
}

#[macroquad::main("Rapier + Macroquad Bouncing Ball")]
async fn main() {
    // initialize game state
    let mut state = GameState::Title;
    let mut physics = PhysicsWorld::new();
    let mut camera = Camera::new();
    let mut player = Player::spawn(&mut physics);

    // === variables for game scores ===
    let mut current_score = 0.0;
    let mut high_score = 0.0;
    let mut physics_accumulator = 0.0;

    //load a texture file
    let texture = load_texture("assets/ferris.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    // calculate the scale
    let tex_scale =
        ((BALL_RADIUS + ARM_RADIUS + ARM_HALF_HEIGHT) * 2.0 * SCALE) / texture.width() as f32;

    loop {
        clear_background(LIGHTGRAY);

        match state {
            GameState::Title => {
                draw_text("FERRIS BOUNCER", 40.0, 100.0, 50.0, DARKGRAY);
                draw_text("Press [ENTER] to start", 40.0, 160.0, 25.0, GRAY);

                if is_key_pressed(KeyCode::Enter) {
                    player = reset_game(&mut physics, &mut camera);
                    current_score = 0.0;
                    physics_accumulator = 0.0;
                    state = GameState::Playing;
                }
            }

            GameState::Playing => {
                let frame_time = get_frame_time();
                let simulation_delta = frame_time.min(MAX_FRAME_TIME);

                for _ in 0..consume_physics_steps(&mut physics_accumulator, frame_time) {
                    player.handle_input(&mut physics);
                    physics.step();
                    player.enforce_speed_limit(&mut physics);
                    current_score += TIME_DELTA;
                }

                let body = physics.bodies.get(player.handle).unwrap();
                let pos = body.translation();
                let angle = body.rotation().angle();

                if is_key_down(KeyCode::Escape) {
                    state = GameState::Title;
                } else if pos.y < LOWER_WORLD_BOUND || PI - angle.abs() < ANGLE_THRESHOLD {
                    if current_score > high_score {
                        high_score = current_score;
                    }
                    state = GameState::GameOver;
                }

                camera.update(*pos, simulation_delta);

                draw_stage(camera.pos);
                draw_player(pos, angle, &texture, tex_scale, camera.pos);

                draw_text(
                    format!("TIME: {:.2}s", current_score).as_str(),
                    20.0,
                    40.0,
                    30.0,
                    BLACK,
                );
            }

            GameState::GameOver => {
                draw_stage(camera.pos);

                draw_rectangle(
                    0.0,
                    0.0,
                    screen_width(),
                    screen_height(),
                    Color::new(0.3, 0.0, 0.0, 0.5),
                );

                draw_text("GAME OVER", 40.0, 100.0, 60.0, RED);
                draw_text(
                    format!("YOUR SCORE: {:.2}s", current_score).as_str(),
                    40.0,
                    150.0,
                    25.0,
                    WHITE,
                );
                draw_text(
                    format!("HIGH SCORE: {:.2}s", current_score).as_str(),
                    40.0,
                    180.0,
                    25.0,
                    ORANGE,
                );

                draw_text("Press [R] to Restart", 40.0, 230.0, 25.0, WHITE);
                draw_text("Press [ESC] for Title", 40.0, 260.0, 25.0, LIGHTGRAY);

                if is_key_pressed(KeyCode::R) {
                    player = reset_game(&mut physics, &mut camera);
                    current_score = 0.0;
                    physics_accumulator = 0.0;
                    state = GameState::Playing;
                } else if is_key_pressed(KeyCode::Escape) {
                    state = GameState::Title;
                }
            }
        }

        next_frame().await;
    }
}

#[cfg(test)]
mod tests;
