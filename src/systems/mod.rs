pub mod camera;
pub mod input;
pub mod physics;
pub mod render;
pub mod spawn;

pub use camera::camera_update_system;
pub use input::player_input_system;
pub use physics::{enforce_speed_limit_system, physics_step_system};
pub use render::{draw_player_system, draw_stage_system};
pub use spawn::{spawn_player, spawn_stage};
