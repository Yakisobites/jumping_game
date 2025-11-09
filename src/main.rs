use macroquad::prelude::*;
use rapier2d::prelude::*;

const SCALE: f32 = 50.0; // [pixel/m]
// const CAMERA_WIDTH:f32 = 100.0; // [pixel]
const CAMERA_HEIGHT:f32 = 100.0; // [pixel]
const BALL_RADIUS: f32 = 0.3; // [m]
const ARM_HALF_HEIGHT: f32 = 0.4; // [m]
const ARM_RADIUS: f32 = 0.25; // [m]
const FLOAR_THICKNESS: f32 = 0.5; // [m]
const FLOAR_WIDTH: f32 = 100.0; // [m]
const TIME_DELTA: f32 = 1.0 / 1200.0; // [1/s]
const KP: f32 = 50.0;
const KD: f32 = 5.0;

const PI: f32 = std::f32::consts::PI;
const LOWER_WORLD_BOUND: f32 = -50.0; // [m]
const ANGLE_THRESHOLD: f32 = 0.2; // [rad]


// === 座標変換 ===
// Rapierの世界座標（メートル単位）を画面座標（ピクセル）に変換
/*
    input: v: Vec4 (CenterX, CenterY, HalfWidth, HalfHeight)
    output: Vec4 (TopLeftX, TopLeftY, Width, Height)
*/
fn world_to_screen_cubiod(v: Vec4, camera_pos: Vector<f32>) -> Vec4 {
    // y軸を上下反転して、中心を画面中央に
    vec4((v.x - v.z - camera_pos[0])*SCALE + screen_width() / 2.0, screen_height() - (v.y + v.w - camera_pos[1])*SCALE - CAMERA_HEIGHT, 2.0*v.z*SCALE, 2.0*v.w*SCALE)
}

fn world_to_screen(v: Vector<f32>, camera_pos: Vector<f32>) -> Vec2 {
    // y軸を上下反転して、中心を画面中央に
    vec2(v.x * SCALE + screen_width() / 2.0 - camera_pos[0] * SCALE, screen_height() - v.y * SCALE - CAMERA_HEIGHT + camera_pos[1] * SCALE)
}

#[macroquad::main("Rapier + Macroquad Bouncing Ball")]
async fn main() {
    // === Rapier 構造体初期化 ===
    let gravity = vector![0.0, -9.81];
    let mut integration_parameters = IntegrationParameters::default();
    integration_parameters.dt = TIME_DELTA;
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
    let ground = ColliderBuilder::cuboid(FLOAR_WIDTH, FLOAR_THICKNESS / 2.0).build();
    collider_set.insert(ground);

    // === オブジェクトを作成 ===
    let object = ColliderBuilder::cuboid(1.0, 5.0 / 2.0).translation(vector![5.0, 5.0 / 2.0]).build();
    collider_set.insert(object);

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

    let tex_scale = ((BALL_RADIUS + ARM_RADIUS + ARM_HALF_HEIGHT) * 2.0 * SCALE) / texture.width() as f32;

    let mut camera_pos = vector![0.0, 0.0];
    let mut camera_vel = vector![0.0, 0.0];
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
        let ball = rigid_body_set.get_mut(ball_handle).unwrap();
        let pos = ball.translation();
        let angle = ball.rotation().angle(); // 角度
        let screen_pos = world_to_screen(*pos, camera_pos);
        draw_texture_ex(
            &texture,
            screen_pos.x - texture.width() as f32 * tex_scale / 2.0,
            screen_pos.y - texture.height() as f32 * tex_scale / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(texture.width() as f32 * tex_scale, texture.height() as f32 * tex_scale)),
                rotation: angle, // 回転角を適用
                ..Default::default()
            },
        );

        // --- 床を描画 ---
        let ground_plane_params = world_to_screen_cubiod(vec4(0.0, 0.0, FLOAR_WIDTH, FLOAR_THICKNESS / 2.0), camera_pos);
        draw_rectangle(ground_plane_params.x, ground_plane_params.y, ground_plane_params.z, ground_plane_params.w, DARKGRAY);


        // オブジェクトを描画
        let object_params = world_to_screen_cubiod(vec4(5.0, 5.0 / 2.0, 1.0, 5.0 / 2.0), camera_pos);
        draw_rectangle(object_params.x, object_params.y, object_params.z, object_params.w, BLUE);
        // --- 高さをコンソールに出す（デバッグ） ---
        // println!("Ball altitude: {:.2}", pos.y);
        // println!("Ball placement: {:.2}", pos.x);
        // cameraをFerrisくんにフォーカスするようにする
        let target = vector![pos.x, pos.y];
        let error = target - camera_pos;
        let force = error * KP - camera_vel * KD;

        camera_vel += force * TIME_DELTA;
        camera_pos += camera_vel * TIME_DELTA;

        // --- ESCキーで終了 ---
        if is_key_down(KeyCode::Escape) {
            break;
        }

        // 高さ下限
        if pos.y < LOWER_WORLD_BOUND {
            break;
        }

        // 角度制限
        if PI - angle.abs() < ANGLE_THRESHOLD {
            break;
        }

        // 速度上限
        let vel = ball.linvel();
        let speed = vel.norm();

        if speed > 10.0 {
            let keep_vel = vel / speed * 10.0;
            ball.set_linvel(keep_vel, true);
        }

        if is_key_down(KeyCode::Up) {
            ball.reset_forces(true); // Reset the forces to zero.
            ball.reset_torques(true); // Reset the torques to zero.

            ball.apply_impulse(vector![0.0, 0.05], true);
        }

        if is_key_down(KeyCode::Left) {
            ball.reset_forces(true); // Reset the forces to zero.
            ball.reset_torques(true); // Reset the torques to zero.

            ball.apply_impulse(vector![-0.03, 0.0], true);
        }

        if is_key_down(KeyCode::Right) {
            ball.reset_forces(true); // Reset the forces to zero.
            ball.reset_torques(true); // Reset the torques to zero.

            ball.apply_impulse(vector![0.03, 0.0], true);
        }
        next_frame().await;
    }
}
