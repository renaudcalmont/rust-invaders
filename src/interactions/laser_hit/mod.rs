use bevy::prelude::*;

mod systems;

pub(crate) struct LaserHitPlugin;

impl Plugin for LaserHitPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(systems::player_laser_hit_enemy)
			.add_system(systems::enemy_laser_hit_player);
	}
}
