use crate::{config::*, physics::PhysicsWorld};
use macroquad::prelude::*;
use rapier2d::prelude::*;

// === ステージ環境の生成 ===
pub fn spawn_stage(world: &mut PhysicsWorld) {
    // 床を作成
    let ground = ColliderBuilder::cuboid(FLOOR_WIDTH, FLOOR_THICKNESS / 2.0).build();
    world.colliders.insert(ground);

    // オブジェクトを作成
    let object = ColliderBuilder::cuboid(1.0, 5.0 / 2.0)
        .translation(vector![5.0, 5.0 / 2.0])
        .build();
    world.colliders.insert(object);
}

// === プレイヤー（Ferris君）の管理 ===
pub struct Player {
    pub handle: RigidBodyHandle,
}

impl Player {
    pub fn spawn(world: &mut PhysicsWorld) -> Self {
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

        let handle = world.bodies.insert(ball_body);
        world
            .colliders
            .insert_with_parent(ball_collider, handle, &mut world.bodies);
        world
            .colliders
            .insert_with_parent(left_arm, handle, &mut world.bodies);
        world
            .colliders
            .insert_with_parent(right_arm, handle, &mut world.bodies);

        Self { handle }
    }

    pub fn handle_input(&self, world: &mut PhysicsWorld) {
        let body = world.bodies.get_mut(self.handle).unwrap();
        let input_scale = world.integration_parameters.dt / LEGACY_TIME_DELTA;

        if is_key_down(KeyCode::Up) {
            body.reset_forces(true);
            body.reset_torques(true);
            body.apply_impulse(vector![0.0, 0.05 * input_scale], true);
        }
        if is_key_down(KeyCode::Left) {
            body.reset_forces(true);
            body.reset_torques(true);
            body.apply_impulse(vector![-0.03 * input_scale, 0.0], true);
        }
        if is_key_down(KeyCode::Right) {
            body.reset_forces(true);
            body.reset_torques(true);
            body.apply_impulse(vector![0.03 * input_scale, 0.0], true);
        }
    }

    pub fn enforce_speed_limit(&self, world: &mut PhysicsWorld) {
        let body = world.bodies.get_mut(self.handle).unwrap();
        let vel = body.linvel();
        let speed = vel.norm();

        if speed > MAX_SPEED {
            let keep_vel = vel / speed * MAX_SPEED;
            body.set_linvel(keep_vel, true);
        }
    }
}
