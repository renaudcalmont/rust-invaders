use bevy::prelude::*;
use bevy::time::FixedTimestep;

pub(crate) mod components;
pub(crate) mod events;
mod resources;
mod systems;

pub(crate) struct PlayerPlugin;
const PLAYER_RESPAWN_DELAY: f64 = 2.;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(resources::setup)
			.add_startup_system(systems::spawn_camera)
			.add_event::<events::PlayerHit>()
			.add_event::<events::PlayerKilled>()
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(0.5))
					.with_system(systems::spawn_player),
			)
			.add_system(systems::player_fire)
			.add_system(systems::player_move)
			.add_system(systems::player_hit_listener);
	}
}
