use bevy::prelude::*;

pub(crate) mod components;
pub(crate) mod resources;
mod systems;

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(systems::apply_movement);
	}
}
