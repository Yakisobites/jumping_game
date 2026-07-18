use crate::{config::*, consume_physics_steps};

#[test]
fn consume_physics_steps_carries_fractional_time() {
    let mut accumulator = 0.0;

    let steps = consume_physics_steps(&mut accumulator, TIME_DELTA * 2.5);

    assert_eq!(steps, 2);
    assert!((accumulator - TIME_DELTA * 0.5).abs() < f32::EPSILON);
}

#[test]
fn consume_physics_steps_clamps_large_frame_spikes() {
    let mut accumulator = 0.0;

    let steps = consume_physics_steps(&mut accumulator, MAX_FRAME_TIME * 10.0);

    assert_eq!(steps, MAX_PHYSICS_STEPS_PER_FRAME);
    assert_eq!(accumulator, 0.0);
}
