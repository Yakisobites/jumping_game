mod components;
mod config;
mod resources;
mod systems;

use config::*;
use hecs::World;
use macroquad::prelude::*;
use resources::{Camera, GameAssets, GameScore, PhysicsWorld};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Title,
    Playing,
    GameOver,
}

// ゲームの物理ワールド・カメラ・ECS Worldをリセットし、プレイヤーを再スポーンする
fn reset_game(world: &mut World, physics: &mut PhysicsWorld, camera: &mut Camera) {
    *world = World::new();
    *physics = PhysicsWorld::new();
    *camera = Camera::new();

    systems::spawn_stage(physics);
    systems::spawn_player(world, physics);
}

// フレーム時間を固定タイムステップに変換し、実行すべきステップ数を返す
pub fn consume_physics_steps(accumulator: &mut f32, frame_time: f32) -> usize {
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
    let mut state = GameState::Title;
    let mut world = World::new();
    let mut physics = PhysicsWorld::new();
    let mut camera = Camera::new();
    let mut score = GameScore::new();

    // 初期スポーン
    systems::spawn_stage(&mut physics);
    systems::spawn_player(&mut world, &mut physics);

    // テクスチャの読み込みとスケール計算
    let texture = load_texture("assets/ferris.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let tex_scale =
        ((BALL_RADIUS + ARM_RADIUS + ARM_HALF_HEIGHT) * 2.0 * SCALE) / texture.width() as f32;
    let assets = GameAssets { texture, tex_scale };

    loop {
        clear_background(LIGHTGRAY);

        match state {
            GameState::Title => {
                draw_text("FERRIS BOUNCER", 40.0, 100.0, 50.0, DARKGRAY);
                draw_text("Press [ENTER] to start", 40.0, 160.0, 25.0, GRAY);

                if is_key_pressed(KeyCode::Enter) {
                    reset_game(&mut world, &mut physics, &mut camera);
                    score.reset();
                    state = GameState::Playing;
                }
            }

            GameState::Playing => {
                let frame_time = get_frame_time();
                let simulation_delta = frame_time.min(MAX_FRAME_TIME);

                for _ in 0..consume_physics_steps(&mut score.physics_accumulator, frame_time) {
                    systems::player_input_system(&mut world, &mut physics);
                    systems::physics_step_system(&mut world, &mut physics);
                    systems::enforce_speed_limit_system(&mut world, &mut physics);
                    score.current += TIME_DELTA;
                }

                // プレイヤーの位置・角度をECSから読み取ってゲームオーバー判定
                let mut player_pos_y = 0.0_f32;
                let mut player_angle = 0.0_f32;
                for (_tag, pos, rot) in world.query_mut::<(
                    &components::PlayerTag,
                    &components::Position,
                    &components::Rotation,
                )>() {
                    player_pos_y = pos.y;
                    player_angle = rot.angle;
                }

                if is_key_down(KeyCode::Escape) {
                    state = GameState::Title;
                } else if player_pos_y < LOWER_WORLD_BOUND
                    || PI - player_angle.abs() < ANGLE_THRESHOLD
                {
                    if score.current > score.high {
                        score.high = score.current;
                    }
                    state = GameState::GameOver;
                }

                systems::camera_update_system(&mut world, &mut camera, simulation_delta);
                systems::draw_stage_system(&camera);
                systems::draw_player_system(&mut world, &camera, &assets);

                draw_text(
                    format!("TIME: {:.2}s", score.current).as_str(),
                    20.0,
                    40.0,
                    30.0,
                    BLACK,
                );
            }

            GameState::GameOver => {
                systems::draw_stage_system(&camera);

                draw_rectangle(
                    0.0,
                    0.0,
                    screen_width(),
                    screen_height(),
                    Color::new(0.3, 0.0, 0.0, 0.5),
                );

                draw_text("GAME OVER", 40.0, 100.0, 60.0, RED);
                draw_text(
                    format!("YOUR SCORE: {:.2}s", score.current).as_str(),
                    40.0,
                    150.0,
                    25.0,
                    WHITE,
                );
                draw_text(
                    format!("HIGH SCORE: {:.2}s", score.high).as_str(),
                    40.0,
                    180.0,
                    25.0,
                    ORANGE,
                );

                draw_text("Press [R] to Restart", 40.0, 230.0, 25.0, WHITE);
                draw_text("Press [ESC] for Title", 40.0, 260.0, 25.0, LIGHTGRAY);

                if is_key_pressed(KeyCode::R) {
                    reset_game(&mut world, &mut physics, &mut camera);
                    score.reset();
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
