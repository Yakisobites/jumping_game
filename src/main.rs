use macroquad::prelude::*;
use rapier2d::prelude::*;

const SCALE: f32 = 50.0; // [pixel/m]
const CAMERA_HEIGHT:f32 = 100.0; // [pixel]
const BALL_RADIUS: f32 = 0.5; // [m]
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

    // === ボール作成 ===
    let ball_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 10.0])
        .build();
    let ball_collider = ColliderBuilder::ball(BALL_RADIUS).restitution(0.8).build();

    let ball_handle = rigid_body_set.insert(ball_body);
    collider_set.insert_with_parent(ball_collider, ball_handle, &mut rigid_body_set);

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
        let screen_pos = world_to_screen(*pos);
        draw_circle(screen_pos.x, screen_pos.y, BALL_RADIUS * SCALE, RED);

        // --- 床を描画 ---
        let ground_y = world_to_screen(vector![0.0, 0.0]).y;
        draw_rectangle(0.0, ground_y + (FLOAR_THICKNESS * SCALE) / 2.0, screen_width(), - FLOAR_THICKNESS * SCALE, DARKGRAY);

        // --- 高さをコンソールに出す（デバッグ） ---
        println!("Ball altitude: {:.2}", pos.y);

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

            ball.apply_impulse(vector![-0.01, 0.0], true);
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

            ball.apply_impulse(vector![0.01, 0.0], true);
            // ball.apply_torque_impulse(100.0, true);
            // ball.apply_impulse_at_point(vector![0.0, 10.0], point![1.0, 2.0], true);
        }
        // ball.reset_forces(true); // Reset the forces to zero.
        // ball.reset_torques(true); // Reset the torques to zero.

        next_frame().await;
    }
}
