use crate::config::*;
use macroquad::prelude::*;
use rapier2d::prelude::*;

// === 座標変換ユーティリティ ===
pub fn world_to_screen_cuboid(v: Vec4, camera_pos: Vector<f32>) -> Vec4 {
    vec4(
        (v.x - v.z - camera_pos[0]) * SCALE + screen_width() / 2.0,
        screen_height() - (v.y + v.w - camera_pos[1]) * SCALE - CAMERA_HEIGHT,
        2.0 * v.z * SCALE,
        2.0 * v.w * SCALE,
    )
}

pub fn world_to_screen(v: Vector<f32>, camera_pos: Vector<f32>) -> Vec2 {
    vec2(
        v.x * SCALE + screen_width() / 2.0 - camera_pos[0] * SCALE,
        screen_height() - v.y * SCALE - CAMERA_HEIGHT + camera_pos[1] * SCALE,
    )
}

// === カメラ制御 ===
pub struct Camera {
    pub pos: Vector<f32>,
    pub vel: Vector<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: vector![0.0, 0.0],
            vel: vector![0.0, 0.0],
        }
    }

    pub fn update(&mut self, target_pos: Vector<f32>, delta_time: f32) {
        let error = target_pos - self.pos;
        let force = error * KP - self.vel * KD;

        self.vel += force * delta_time;
        self.pos += self.vel * delta_time;
    }
}

// === 描画ロジック ===
pub fn draw_stage(camera_pos: Vector<f32>) {
    // 床の描画
    let ground_params = world_to_screen_cuboid(
        vec4(0.0, 0.0, FLOOR_WIDTH, FLOOR_THICKNESS / 2.0),
        camera_pos,
    );
    draw_rectangle(
        ground_params.x,
        ground_params.y,
        ground_params.z,
        ground_params.w,
        DARKGRAY,
    );

    // オブジェクトの描画
    let object_params = world_to_screen_cuboid(vec4(5.0, 5.0 / 2.0, 1.0, 5.0 / 2.0), camera_pos);
    draw_rectangle(
        object_params.x,
        object_params.y,
        object_params.z,
        object_params.w,
        BLUE,
    );
}

pub fn draw_player(
    pos: &Vector<f32>,
    angle: f32,
    texture: &Texture2D,
    tex_scale: f32,
    camera_pos: Vector<f32>,
) {
    let screen_pos = world_to_screen(*pos, camera_pos);

    draw_texture_ex(
        texture,
        screen_pos.x - texture.width() * tex_scale / 2.0,
        screen_pos.y - texture.height() * tex_scale / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                texture.width() * tex_scale,
                texture.height() * tex_scale,
            )),
            rotation: angle,
            ..Default::default()
        },
    );
}
