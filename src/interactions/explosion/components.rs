use bevy::prelude::Component;
use bevy::time::Timer;

#[derive(Component)]
pub(super) struct Explosion {
	pub timer: Timer,
}

impl Default for Explosion {
	fn default() -> Self {
		Self {
			timer: Timer::from_seconds(0.05, true),
		}
	}
}
