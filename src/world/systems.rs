use super::components::{Movable, Velocity};
use super::{BASE_SPEED, TIME_STEP};
use bevy::prelude::*;

pub(super) fn apply_movement(
	mut commands: Commands,
	mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
	mut windows: ResMut<Windows>,
) {
	let window = windows.get_primary_mut().unwrap();
	let window_height = window.height();
	let window_width = window.width();
	for (entity, velocity, mut transform, movable) in query.iter_mut() {
		let translation = &mut transform.translation;
		translation.x += velocity.translation.x * TIME_STEP * BASE_SPEED;
		translation.y += velocity.translation.y * TIME_STEP * BASE_SPEED;

		if movable.auto_despawn {
			// despawn when out of screen
			const MARGIN: f32 = 200.;
			if translation.y > window_height / 2. + MARGIN
				|| translation.y < -window_height / 2. - MARGIN
				|| translation.x > window_width / 2. + MARGIN
				|| translation.x < -window_width / 2. - MARGIN
			{
				commands.entity(entity).despawn();
			}
		}
	}
}
