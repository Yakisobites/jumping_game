use macroquad::prelude::*;
use rapier2d::prelude::*;

const SCALE: f32 = 50.0; // [pixel/m]
const CAMERA_HEIGHT:f32 = 100.0; // [pixel]
const BALL_RADIUS: f32 = 0.3; // [m]
const ARM_HALF_HEIGHT: f32 = 0.4; // [m]
const ARM_RADIUS: f32 = 0.50; // [m]
const FLOAR_THICKNESS: f32 = 0.2; // [m]

// === 座標変換 ===
// Rapierの世界座標（メートル単位）を画面座標（ピクセル）に変換
fn world_to_screen(v: Vector<f32>) -> Vec2 {
    // y軸を上下反転して、中心を画面中央に
    vec2(v.x * SCALE + screen_width() / 2.0, screen_height() - v.y * SCALE - CAMERA_HEIGHT)
}

#[macroquad::main("Rapier + Macroquad Bouncing Ball")]
async fn main() {
    // === Rapier 構造体初期化 ===
    let gravity = vector![0.0, -9.81];
    let mut integration_parameters = IntegrationParameters::default();
    integration_parameters.dt = 1.0 / 1200.0;
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = DefaultBroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    // === 床を作成 ===
    let ground = ColliderBuilder::cuboid(100.0, FLOAR_THICKNESS / 2.0).build();
    collider_set.insert(ground);

    // === Ferris君作成 ===
    // 胴体の作成
    let ball_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 10.0])
        .build();
    let ball_collider = ColliderBuilder::ball(BALL_RADIUS).restitution(0.8).build();
    // 左側のカプセル（x軸方向）
    let left_arm = ColliderBuilder::capsule_x(ARM_HALF_HEIGHT, ARM_RADIUS)
        .translation(vector![-(BALL_RADIUS + ARM_RADIUS), 0.0]) // 左にオフセット
        .build();

    // 右側のカプセル（x軸方向）
    let right_arm = ColliderBuilder::capsule_x(ARM_HALF_HEIGHT, ARM_RADIUS)
        .translation(vector![(BALL_RADIUS + ARM_RADIUS), 0.0]) // 右にオフセット
        .build();

    let ball_handle = rigid_body_set.insert(ball_body);
    collider_set.insert_with_parent(ball_collider, ball_handle, &mut rigid_body_set);
    collider_set.insert_with_parent(left_arm, ball_handle, &mut rigid_body_set);
    collider_set.insert_with_parent(right_arm, ball_handle, &mut rigid_body_set);

    let texture = load_texture("assets/ferris.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    let tex_scale = ((BALL_RADIUS + ARM_RADIUS + ARM_HALF_HEIGHT / 2.0) * 2.0 * SCALE) / texture.width() as f32;

    // === メインループ ===
    loop {
        clear_background(LIGHTGRAY);

        // --- 物理シミュレーション更新 ---
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );

        // --- ボールを描画 ---
        // let ball = &rigid_body_set[ball_handle];
        let ball = rigid_body_set.get_mut(ball_handle).unwrap();
        let pos = ball.translation();
        let angle = ball.rotation().angle(); // 角度
        let screen_pos = world_to_screen(*pos);
        draw_texture_ex(
            &texture,
            screen_pos.x - texture.width() as f32 * tex_scale / 2.0,
            screen_pos.y - texture.height() as f32 * tex_scale / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(texture.width() as f32 * tex_scale, texture.height() as f32 * tex_scale)),
                rotation: angle, // ← 回転角を適用！
                // pivot: Some(vec2(texture.width() as f32 * tex_scale / 2.0, texture.height() as f32 * tex_scale / 2.0)), // 中心回転
                ..Default::default()
            },
        );

        // --- 床を描画 ---
        let ground_y = world_to_screen(vector![0.0, - FLOAR_THICKNESS / 2.0]).y;
        draw_rectangle(0.0, ground_y, screen_width(), FLOAR_THICKNESS * SCALE, DARKGRAY);
        // // --- 高さをコンソールに出す（デバッグ） ---
        // println!("Ball altitude: {:.2}", pos.y);
        // println!("Ball rotation: {:.2}", angle);

        // --- ESCキーで終了 ---
        if is_key_down(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::Up) {
            ball.reset_forces(true); // Reset the forces to zero.
            ball.reset_torques(true); // Reset the torques to zero.
            // ball.add_force(vector![0.0, 10.0], true);
            // ball.add_torque(0.0, true);
            // ball.add_force_at_point(vector![0.0, 10.0], point![1.0, 2.0], true);

            ball.apply_impulse(vector![0.0, 0.05], true);
            // ball.apply_torque_impulse(100.0, true);
            // ball.apply_impulse_at_point(vector![0.0, 10.0], point![1.0, 2.0], true);
        }

        if is_key_down(KeyCode::Left) {
            ball.reset_forces(true); // Reset the forces to zero.
            ball.reset_torques(true); // Reset the torques to zero.
            // ball.add_force(vector![0.0, 10.0], true);
            // ball.add_torque(0.0, true);
            // ball.add_force_at_point(vector![0.0, 10.0], point![1.0, 2.0], true);

            ball.apply_impulse(vector![-0.03, 0.0], true);
            // ball.apply_torque_impulse(100.0, true);
            // ball.apply_impulse_at_point(vector![0.0, 10.0], point![1.0, 2.0], true);
        }
        // ball.reset_forces(true); // Reset the forces to zero.
        // ball.reset_torques(true); // Reset the torques to zero.

        if is_key_down(KeyCode::Right) {
            ball.reset_forces(true); // Reset the forces to zero.
            ball.reset_torques(true); // Reset the torques to zero.
            // ball.add_force(vector![0.0, 10.0], true);
            // ball.add_torque(0.0, true);
            // ball.add_force_at_point(vector![0.0, 10.0], point![1.0, 2.0], true);

            ball.apply_impulse(vector![0.03, 0.0], true);
            // ball.apply_torque_impulse(100.0, true);
            // ball.apply_impulse_at_point(vector![0.0, 10.0], point![1.0, 2.0], true);
        }
        // ball.reset_forces(true); // Reset the forces to zero.
        // ball.reset_torques(true); // Reset the torques to zero.

        next_frame().await;
    }
}
