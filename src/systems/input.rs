use crate::{
    components::{PhysicsBody, PlayerTag},
    config::*,
    resources::PhysicsWorld,
};
use hecs::World;
use macroquad::prelude::*;
use rapier2d::prelude::*;

// プレイヤーのキー入力を読み取り、物理ボディに力を適用する
pub fn player_input_system(world: &mut World, physics: &mut PhysicsWorld) {
    for (_tag, body_ref) in world.query_mut::<(&PlayerTag, &PhysicsBody)>() {
        let body = physics.bodies.get_mut(body_ref.handle).unwrap();
        let input_scale = physics.integration_parameters.dt / LEGACY_TIME_DELTA;

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
}
