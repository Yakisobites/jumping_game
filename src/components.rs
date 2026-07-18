use rapier2d::prelude::RigidBodyHandle;

// ワールド座標における位置
pub struct Position {
    pub x: f32,
    pub y: f32,
}

// 回転角度（ラジアン）
pub struct Rotation {
    pub angle: f32,
}

// 物理エンジンのリジッドボディとのリンク
pub struct PhysicsBody {
    pub handle: RigidBodyHandle,
}

// プレイヤーエンティティを識別するマーカー
pub struct PlayerTag;
