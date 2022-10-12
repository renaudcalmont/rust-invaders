use bevy::prelude::*;

pub(crate) struct PlayerHit {
	pub damage: u32,
	pub translation: Vec3,
}

pub(crate) struct PlayerKilled {
	pub translation: Vec3,
}
