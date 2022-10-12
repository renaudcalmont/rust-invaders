use bevy::prelude::*;

pub(crate) struct EnemyHit {
	pub entity: Entity,
	pub damage: u32,
	pub translation: Vec3,
}

pub(crate) struct EnemyKilled {
	pub translation: Vec3,
}
