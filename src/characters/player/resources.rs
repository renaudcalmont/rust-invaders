use bevy::prelude::*;

pub(super) const PLAYER_SIZE: Vec3 = Vec3::new(144., 75., 0.);
pub(super) const LASER_SIZE: Vec3 = Vec3::new(9., 54., 0.);

pub(crate) struct PlayerState {
	pub alive: bool,
	pub last_shot: f64, // -1 if not shot
}

impl Default for PlayerState {
	fn default() -> Self {
		Self {
			alive: false,
			last_shot: -1.,
		}
	}
}

impl PlayerState {
	pub fn shot(&mut self, time: f64) {
		self.alive = false;
		self.last_shot = time;
	}
	pub fn spawned(&mut self) {
		self.alive = true;
		self.last_shot = -1.;
	}
}

pub(super) struct PlayerAssets {
	pub player_image: Handle<Image>,
	pub laser_image: Handle<Image>,
	pub laser_sound: Handle<AudioSource>,
}

pub(super) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.insert_resource(PlayerState::default());
	commands.insert_resource(PlayerAssets {
		player_image: asset_server.load("player_b_01.png"),
		laser_image: asset_server.load("laser_a_01.png"),
		laser_sound: asset_server.load("mixkit-short-laser-gun-shot-1670.flac"),
	})
}
