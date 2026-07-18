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

// 1. Configure game state
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

#[macroquad::main("Rapier + Macroquad Bouncing Ball")]
async fn main() {
    // 初期化
    let mut state = GameState::Title;
    let mut physics = PhysicsWorld::new();
    let mut camera = Camera::new();

    let mut player = Player::spawn(&mut physics);

    // テクスチャの読み込みとスケール計算
    let texture = load_texture("assets/ferris.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let tex_scale =
        ((BALL_RADIUS + ARM_RADIUS + ARM_HALF_HEIGHT) * 2.0 * SCALE) / texture.width() as f32;

    // メインループ
    loop {
        clear_background(LIGHTGRAY);

        match state {
            GameState::Title => {
                draw_text("FERRIS BOUNCER", 40.0, 100.0, 50.0, DARKGRAY);
                draw_text("Press [ENTER] to start", 40.0, 160.0, 25.0, GRAY);

                if is_key_pressed(KeyCode::Enter) {
                    player = reset_game(&mut physics, &mut camera);
                    state = GameState::Playing;
                }
            }

            GameState::Playing => {
                player.handle_input(&mut physics);
                physics.step();
                player.enforce_speed_limit(&mut physics);

                let body = physics.bodies.get(player.handle).unwrap();
                let pos = body.translation();
                let angle = body.rotation().angle();

                if is_key_down(KeyCode::Escape) {
                    state = GameState::Title;
                } else if pos.y < LOWER_WORLD_BOUND || PI - angle.abs() < ANGLE_THRESHOLD {
                    state = GameState::GameOver;
                }

                camera.update(*pos);

                draw_stage(camera.pos);
                draw_player(pos, angle, &texture, tex_scale, camera.pos);
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
                draw_text("Press [R] to Restart", 40.0, 180.0, 25.0, WHITE);
                draw_text("Press [ESC] for Title", 40.0, 220.0, 25.0, LIGHTGRAY);

                if is_key_pressed(KeyCode::R) {
                    player = reset_game(&mut physics, &mut camera);
                    state = GameState::Playing;
                } else if is_key_pressed(KeyCode::Escape) {
                    state = GameState::Title;
                }
            }
        }

        next_frame().await;
    }
}
