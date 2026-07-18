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

#[macroquad::main("Rapier + Macroquad Bouncing Ball")]
async fn main() {
    // 初期化
    let mut physics = PhysicsWorld::new();
    let mut camera = Camera::new();

    spawn_stage(&mut physics);
    let player = Player::spawn(&mut physics);

    // テクスチャの読み込みとスケール計算
    let texture = load_texture("assets/ferris.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let tex_scale =
        ((BALL_RADIUS + ARM_RADIUS + ARM_HALF_HEIGHT) * 2.0 * SCALE) / texture.width() as f32;

    // メインループ
    loop {
        clear_background(LIGHTGRAY);

        // 1. 入力処理
        player.handle_input(&mut physics);

        // 2. 物理演算のステップ実行
        physics.step();
        player.enforce_speed_limit(&mut physics);

        // 3. プレイヤーの状態を取得
        let body = physics.bodies.get(player.handle).unwrap();
        let pos = body.translation();
        let angle = body.rotation().angle();

        // 4. 終了判定
        if is_key_down(KeyCode::Escape)
            || pos.y < LOWER_WORLD_BOUND
            || PI - angle.abs() < ANGLE_THRESHOLD
        {
            break;
        }

        // 5. カメラ追従の更新
        camera.update(*pos);

        // 6. 描画
        draw_stage(camera.pos);
        draw_player(pos, angle, &texture, tex_scale, camera.pos);

        next_frame().await;
    }
}
