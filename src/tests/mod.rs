use crate::{
    components::{PlayerTag, Position, Rotation},
    config::*,
    consume_physics_steps,
    systems::check_goal_system,
};
use hecs::World;

#[test]
fn consume_physics_steps_carries_fractional_time() {
    let mut accumulator = 0.0;

    let steps = consume_physics_steps(&mut accumulator, TIME_DELTA * 2.5);

    assert_eq!(steps, 2);
    assert!((accumulator - TIME_DELTA * 0.5).abs() < f32::EPSILON);
}

#[test]
fn consume_physics_steps_clamps_large_frame_spikes() {
    let mut accumulator = 0.0;

    let steps = consume_physics_steps(&mut accumulator, MAX_FRAME_TIME * 10.0);

    assert_eq!(steps, MAX_PHYSICS_STEPS_PER_FRAME);
    assert_eq!(accumulator, 0.0);
}

// ゴール領域にプレイヤーがいる場合に check_goal_system が true を返す
#[test]
fn check_goal_system_detects_player_inside_goal() {
    let mut world = World::new();
    world.spawn((
        PlayerTag,
        Position {
            x: GOAL_X,
            y: GOAL_Y,
        },
        Rotation { angle: 0.0 },
    ));

    assert!(check_goal_system(&mut world));
}

// ゴール領域外にプレイヤーがいる場合に check_goal_system が false を返す
#[test]
fn check_goal_system_ignores_player_outside_goal() {
    let mut world = World::new();
    world.spawn((
        PlayerTag,
        Position {
            x: GOAL_X + GOAL_HALF_WIDTH * 3.0,
            y: GOAL_Y,
        },
        Rotation { angle: 0.0 },
    ));

    assert!(!check_goal_system(&mut world));
}

// ゴール境界ちょうどに接触しているプレイヤーを検出できる
#[test]
fn check_goal_system_detects_player_at_goal_edge() {
    let mut world = World::new();
    // プレイヤーがゴールの右端ぎりぎりに接触
    world.spawn((
        PlayerTag,
        Position {
            x: GOAL_X + GOAL_HALF_WIDTH,
            y: GOAL_Y,
        },
        Rotation { angle: 0.0 },
    ));

    // BALL_RADIUS 分だけ余裕があるため接触判定はtrue
    assert!(check_goal_system(&mut world));
}

// ワールドにプレイヤーがいない場合は false を返す
#[test]
fn check_goal_system_returns_false_with_no_player() {
    let mut world = World::new();

    assert!(!check_goal_system(&mut world));
}
