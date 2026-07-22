use crate::{
    components::{GoalTag, PhysicsBody, PlayerTag, Position, Rotation},
    config::*,
    resources::PhysicsWorld,
};
use hecs::World;
use rapier2d::prelude::*;

// ステージの静的コライダーを物理ワールドに登録する
pub fn spawn_stage(physics: &mut PhysicsWorld) {
    let ground = ColliderBuilder::cuboid(FLOOR_WIDTH, FLOOR_THICKNESS / 2.0).build();
    physics.colliders.insert(ground);

    let object = ColliderBuilder::cuboid(1.0, 5.0 / 2.0)
        .translation(vector![5.0, 5.0 / 2.0])
        .build();
    physics.colliders.insert(object);
}

// ゴールエンティティをECS Worldに登録する
pub fn spawn_goal(world: &mut World) {
    world.spawn((
        GoalTag,
        Position {
            x: GOAL_X,
            y: GOAL_Y,
        },
    ));
}

// プレイヤーエンティティを物理ワールドとECS Worldの両方に登録する
pub fn spawn_player(world: &mut World, physics: &mut PhysicsWorld) -> hecs::Entity {
    let ball_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 10.0])
        .build();
    let ball_collider = ColliderBuilder::ball(BALL_RADIUS).restitution(0.8).build();
    let left_arm = ColliderBuilder::capsule_x(ARM_HALF_HEIGHT, ARM_RADIUS)
        .translation(vector![-(BALL_RADIUS + ARM_RADIUS), 0.0])
        .build();
    let right_arm = ColliderBuilder::capsule_x(ARM_HALF_HEIGHT, ARM_RADIUS)
        .translation(vector![(BALL_RADIUS + ARM_RADIUS), 0.0])
        .build();

    let handle = physics.bodies.insert(ball_body);
    physics
        .colliders
        .insert_with_parent(ball_collider, handle, &mut physics.bodies);
    physics
        .colliders
        .insert_with_parent(left_arm, handle, &mut physics.bodies);
    physics
        .colliders
        .insert_with_parent(right_arm, handle, &mut physics.bodies);

    world.spawn((
        PlayerTag,
        PhysicsBody { handle },
        Position { x: 0.0, y: 10.0 },
        Rotation { angle: 0.0 },
    ))
}
