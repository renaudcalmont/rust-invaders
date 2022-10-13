use bevy::prelude::*;

pub(crate) mod components;
mod systems;

pub(crate) const TIME_STEP: f32 = 1. / 60.;
pub(crate) const BASE_SPEED: f32 = 500.;

pub(crate) const SPRITE_SCALE: Vec3 = Vec3::new(0.5, 0.5, 1.);

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(systems::apply_movement);
	}
}
