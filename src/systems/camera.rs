use crate::{
    components::{PlayerTag, Position},
    config::*,
    resources::Camera,
};
use hecs::World;
use rapier2d::prelude::*;

// プレイヤーの位置にカメラをPD制御で追従させる
pub fn camera_update_system(world: &mut World, camera: &mut Camera, delta_time: f32) {
    for (_tag, pos) in world.query_mut::<(&PlayerTag, &Position)>() {
        let target = vector![pos.x, pos.y];
        let error = target - camera.pos;
        let force = error * KP - camera.vel * KD;

        camera.vel += force * delta_time;
        camera.pos += camera.vel * delta_time;
    }
}
