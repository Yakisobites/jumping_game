use macroquad::prelude::*;
use rapier2d::prelude::*;

// 画面座標をrapierの座標に変換するヘルパー
fn to_screen_coords(pos: Vector<f32>) -> Vec2 {
    vec2(pos.x * 50.0 + screen_width() / 2.0, screen_height() - pos.y * 50.0 - 100.0)
}

#[macroquad::main("Rapier2D + Macroquad Free Fall")]
async fn main() {
    // === Rapier 物理エンジン初期化 ===
    let gravity = vector![0.0, -9.81];
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut islands = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut ccd_solver = CCDSolver::new();
    let integration_parameters = IntegrationParameters::default();
    let physics_hooks = ();
    let event_handler = ();

    // === 床の作成 ===
    let ground_body = RigidBodyBuilder::fixed()
        .translation(vector![0.0, -3.0])
        .build();
    let ground_handle = bodies.insert(ground_body);
    let ground_collider = ColliderBuilder::cuboid(10.0, 0.2).build();
    colliders.insert_with_parent(ground_collider, ground_handle, &mut bodies);

    // === ボールの作成 ===
    let ball_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 2.0])
        .build();
    let ball_handle = bodies.insert(ball_body);
    let ball_collider = ColliderBuilder::ball(0.3)
        .restitution(0.7)
        .friction(0.5)
        .build();
    colliders.insert_with_parent(ball_collider, ball_handle, &mut bodies);

    // === メインループ ===
    loop {
        clear_background(LIGHTGRAY);

        // Rapierのステップ更新
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut islands,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );

        // ボールの位置を取得して描画
        let ball_body = bodies.get(ball_handle).unwrap();
        let pos = ball_body.translation();
        let screen_pos = to_screen_coords(*pos);
        draw_circle(screen_pos.x, screen_pos.y, 15.0, RED);

        // 床を描画
        let ground_y = to_screen_coords(vector![0.0, -3.0]).y;
        draw_rectangle(0.0, ground_y, screen_width(), 10.0, DARKGRAY);

        // ESCで終了
        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
