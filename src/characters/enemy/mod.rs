use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::{thread_rng, Rng};

use crate::world::components::Health;

pub(crate) mod components;
pub(crate) mod events;
mod resources;
mod systems;

pub(crate) struct EnemyPlugin;
const ENEMY_HEALTH: Health = Health { level: 3 };
pub(super) const ENEMY_MAX_COUNT: u32 = 3;
const FORMATION_MAX_MEMBERS_COUNT: u32 = 1;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(resources::setup)
			.add_event::<events::EnemyHit>()
			.add_event::<events::EnemyKilled>()
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(1.))
					.with_system(systems::spawn_enemy),
			)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(fire_criteria)
					.with_system(systems::enemy_fire),
			)
			.add_system(systems::enemy_move)
			.add_system(systems::enemy_hit_listener)
			.add_system(systems::highlight_animation);
	}
}

fn fire_criteria() -> ShouldRun {
	if thread_rng().gen_bool(1. / 60.) {
		ShouldRun::Yes
	} else {
		ShouldRun::No
	}
}
