use crate::{
    components::{PhysicsBody, PlayerTag, Position, Rotation},
    config::*,
    resources::PhysicsWorld,
};
use hecs::World;

// 物理シミュレーションを1ステップ進め、ECSのPosition/Rotationを同期する
pub fn physics_step_system(world: &mut World, physics: &mut PhysicsWorld) {
    physics.step();

    for (_tag, body_ref, pos, rot) in
        world.query_mut::<(&PlayerTag, &PhysicsBody, &mut Position, &mut Rotation)>()
    {
        let body = physics.bodies.get(body_ref.handle).unwrap();
        let translation = body.translation();
        pos.x = translation.x;
        pos.y = translation.y;
        rot.angle = body.rotation().angle();
    }
}

// プレイヤーの速度を上限以内に制限する
pub fn enforce_speed_limit_system(world: &mut World, physics: &mut PhysicsWorld) {
    for (_tag, body_ref) in world.query_mut::<(&PlayerTag, &PhysicsBody)>() {
        let body = physics.bodies.get_mut(body_ref.handle).unwrap();
        let vel = body.linvel();
        let speed = vel.norm();

        if speed > MAX_SPEED {
            let clamped = vel / speed * MAX_SPEED;
            body.set_linvel(clamped, true);
        }
    }
}

// プレイヤーがゴール領域（AABB）に接触しているか判定する
pub fn check_goal_system(world: &mut World) -> bool {
    for (_tag, pos) in world.query_mut::<(&PlayerTag, &Position)>() {
        if (pos.x - GOAL_X).abs() < GOAL_HALF_WIDTH + BALL_RADIUS
            && (pos.y - GOAL_Y).abs() < GOAL_HALF_HEIGHT + BALL_RADIUS
        {
            return true;
        }
    }
    false
}
