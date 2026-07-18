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
