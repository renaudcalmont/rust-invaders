use bevy::prelude::Component;
use bevy::time::Timer;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct LaserFromEnemy;

#[derive(Clone, Component)]
pub struct Formation {
	pub start: (f32, f32),
	pub radius: (f32, f32),
	pub pivot: (f32, f32),
	pub speed: f32,
	pub angle: f32, // change per tick
}

#[derive(Component)]
pub(super) struct Highlight {
	pub timer: Timer,
}

impl Default for Highlight {
	fn default() -> Self {
		Self {
			timer: Timer::from_seconds(0.1, true),
		}
	}
}
