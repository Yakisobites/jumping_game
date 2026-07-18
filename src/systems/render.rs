use crate::{
    components::{PlayerTag, Position, Rotation},
    config::*,
    resources::{Camera, GameAssets},
};
use hecs::World;
use macroquad::prelude::*;
use rapier2d::prelude::*;

// ワールド座標 (x, y, half_w, half_h) → スクリーン座標 (x, y, w, h) に変換する
fn world_to_screen_cuboid(v: Vec4, camera_pos: Vector<f32>) -> Vec4 {
    vec4(
        (v.x - v.z - camera_pos[0]) * SCALE + screen_width() / 2.0,
        screen_height() - (v.y + v.w - camera_pos[1]) * SCALE - CAMERA_HEIGHT,
        2.0 * v.z * SCALE,
        2.0 * v.w * SCALE,
    )
}

// ワールド座標をスクリーン座標に変換する
fn world_to_screen(v: Vector<f32>, camera_pos: Vector<f32>) -> Vec2 {
    vec2(
        v.x * SCALE + screen_width() / 2.0 - camera_pos[0] * SCALE,
        screen_height() - v.y * SCALE - CAMERA_HEIGHT + camera_pos[1] * SCALE,
    )
}

// ステージ（床・オブジェクト・ゴール）を描画する
pub fn draw_stage_system(camera: &Camera) {
    let ground_params = world_to_screen_cuboid(
        vec4(0.0, 0.0, FLOOR_WIDTH, FLOOR_THICKNESS / 2.0),
        camera.pos,
    );
    draw_rectangle(
        ground_params.x,
        ground_params.y,
        ground_params.z,
        ground_params.w,
        DARKGRAY,
    );

    let object_params = world_to_screen_cuboid(vec4(5.0, 5.0 / 2.0, 1.0, 5.0 / 2.0), camera.pos);
    draw_rectangle(
        object_params.x,
        object_params.y,
        object_params.z,
        object_params.w,
        BLUE,
    );

    let goal_params = world_to_screen_cuboid(
        vec4(GOAL_X, GOAL_Y, GOAL_HALF_WIDTH, GOAL_HALF_HEIGHT),
        camera.pos,
    );
    draw_rectangle(
        goal_params.x,
        goal_params.y,
        goal_params.z,
        goal_params.w,
        GOLD,
    );
    draw_rectangle_lines(
        goal_params.x,
        goal_params.y,
        goal_params.z,
        goal_params.w,
        4.0,
        YELLOW,
    );
}

// プレイヤー（Ferris）をテクスチャで描画する
pub fn draw_player_system(world: &mut World, camera: &Camera, assets: &GameAssets) {
    for (_tag, pos, rot) in world.query_mut::<(&PlayerTag, &Position, &Rotation)>() {
        let screen_pos = world_to_screen(vector![pos.x, pos.y], camera.pos);
        let tex = &assets.texture;
        let s = assets.tex_scale;

        draw_texture_ex(
            tex,
            screen_pos.x - tex.width() * s / 2.0,
            screen_pos.y - tex.height() * s / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(tex.width() * s, tex.height() * s)),
                rotation: rot.angle,
                ..Default::default()
            },
        );
    }
}
