use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(resources::setup)
			.add_system(systems::player_killed_listener)
			.add_system(systems::enemy_killed_listener)
			.add_system(systems::explosion_animation);
	}
}
