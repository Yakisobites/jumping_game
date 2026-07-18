pub const SCALE: f32 = 50.0; // [pixel/m]
pub const CAMERA_HEIGHT: f32 = 100.0; // [pixel]
pub const BALL_RADIUS: f32 = 0.3; // [m]
pub const ARM_HALF_HEIGHT: f32 = 0.4; // [m]
pub const ARM_RADIUS: f32 = 0.25; // [m]
pub const FLOOR_THICKNESS: f32 = 0.5; // [m]
pub const FLOOR_WIDTH: f32 = 100.0; // [m]
pub const TIME_DELTA: f32 = 1.0 / 1200.0; // [1/s]

// PD制御のゲイン
pub const KP: f32 = 50.0;
pub const KD: f32 = 5.0;

// 制限値・閾値
pub const LOWER_WORLD_BOUND: f32 = -50.0; // [m]
pub const ANGLE_THRESHOLD: f32 = 0.2; // [rad]
pub const MAX_SPEED: f32 = 10.0; // [m/s]
pub const GRAVITY: [f32; 2] = [0.0, -9.81];
