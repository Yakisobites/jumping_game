use crate::config::*;
use macroquad::prelude::*;
use rapier2d::prelude::*;

// === 物理ワールド ===
pub struct PhysicsWorld {
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let integration_parameters = IntegrationParameters {
            dt: TIME_DELTA,
            ..Default::default()
        };

        Self {
            integration_parameters,
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }

    pub fn step(&mut self) {
        let gravity = vector![GRAVITY[0], GRAVITY[1]];
        self.physics_pipeline.step(
            &gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            &(),
            &(),
        );
    }
}

// === カメラ（PD制御によるスムーズ追従） ===
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
}

// === ゲームアセット（テクスチャ等） ===
pub struct GameAssets {
    pub texture: Texture2D,
    pub tex_scale: f32,
}

// === スコア・物理ステップ用の状態 ===
pub struct GameScore {
    pub current: f32,
    pub high: f32,
    pub physics_accumulator: f32,
}

impl GameScore {
    pub fn new() -> Self {
        Self {
            current: 0.0,
            high: 0.0,
            physics_accumulator: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.current = 0.0;
        self.physics_accumulator = 0.0;
    }
}
