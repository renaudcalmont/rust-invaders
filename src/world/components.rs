use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub(crate) struct Health {
	pub level: u32,
}

impl Default for Health {
	fn default() -> Self {
		Self { level: 1 }
	}
}

#[derive(Component, Inspectable)]
pub(crate) struct Hitable {
	pub hitbox: Vec3,
}

#[derive(Component, Inspectable)]
pub(crate) struct Movable {
	pub auto_despawn: bool,
}

impl Default for Movable {
	fn default() -> Self {
		Self {
			auto_despawn: Default::default(),
		}
	}
}

#[derive(Component, Inspectable)]
pub(crate) struct Velocity {
	pub translation: Vec3,
	pub rotation: Vec3,
}

impl Default for Velocity {
	fn default() -> Self {
		Self {
			translation: Default::default(),
			rotation: Default::default(),
		}
	}
}
