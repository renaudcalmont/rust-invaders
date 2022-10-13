use crate::world::BASE_SPEED;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use super::components::Formation;

pub(super) const ENEMY_SIZE: Vec3 = Vec3::new(144., 75., 0.);
pub(super) const LASER_SIZE: Vec3 = Vec3::new(9., 54., 0.);

pub(crate) struct EnemyCount(pub u32);

pub(super) struct EnemyAssets {
	pub enemy_image: Handle<Image>,
	pub laser_image: Handle<Image>,
	pub laser_sound: Handle<AudioSource>,
}

pub(super) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.insert_resource(EnemyCount(0));
	commands.insert_resource(EnemyAssets {
		enemy_image: asset_server.load("enemy_a_01.png"),
		laser_image: asset_server.load("laser_b_01.png"),
		laser_sound: asset_server.load("mixkit-game-whip-shot-1512.flac"),
	});
	commands.insert_resource(FormationMaker::default());
}

#[derive(Default)]
pub(super) struct FormationMaker {
	current_template: Option<Formation>,
	current_members: u32,
}

impl FormationMaker {
	pub fn make(&mut self, window_width: f32, window_height: f32) -> Formation {
		match (
			&self.current_template,
			self.current_members >= super::FORMATION_MAX_MEMBERS_COUNT,
		) {
			// if has current template and still within max members
			(Some(tmpl), false) => {
				self.current_members += 1;
				tmpl.clone()
			}
			// if first formation or previous formation is full (need to create a new one)
			(None, _) | (_, true) => {
				let mut rng = thread_rng();

				// compute the start x/y
				let w_span = window_width / 2. + 100.;
				let h_span = window_height / 2. + 100.;
				let x = if rng.gen_bool(0.5) { w_span } else { -w_span };
				let y = rng.gen_range(-h_span..h_span) as f32;
				let start = (x, y);

				// compute the pivot x/y
				let w_span = window_width / 4.;
				let h_span = window_height / 3. - 50.;
				let pivot = (rng.gen_range(-w_span..w_span), rng.gen_range(0.0..h_span));

				// compute the radius
				let radius = (rng.gen_range(80.0..150.), 100.);

				// compute the start angle
				let angle = (y - pivot.1).atan2(x - pivot.0);

				// speed (fixed for now)
				let speed = BASE_SPEED;

				// create the formation
				let formation = Formation {
					start,
					radius,
					pivot,
					speed,
					angle,
				};

				// store as template
				self.current_template = Some(formation.clone());
				// reset members to 1
				self.current_members = 1;

				formation
			}
		}
	}
}
