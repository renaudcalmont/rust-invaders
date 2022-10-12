use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::utils::HashSet;

use crate::characters::{
	enemy::{components::*, events::*},
	player::{components::*, events::*},
};
use crate::world::components::*;

trait AsVec2 {
	fn xy(&self) -> Vec2;
}

impl AsVec2 for Vec3 {
	fn xy(&self) -> Vec2 {
		Vec2::new(self.x, self.y)
	}
}

pub(super) fn player_laser_hit_enemy(
	mut commands: Commands,
	laser_query: Query<(Entity, &Transform, &Hitable), With<LaserFromPlayer>>,
	enemy_query: Query<(Entity, &Transform, &Hitable), With<Enemy>>,
	mut enemy_hit_writer: EventWriter<EnemyHit>,
) {
	let mut despawned_entities: HashSet<Entity> = HashSet::new();

	// iterate through the lasers
	for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
		let laser_scale = laser_tf.scale.xy();

		// iterate through the enemies
		for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
			if despawned_entities.contains(&enemy_entity) {
				continue;
			}

			// determine if collision
			let collision = collide(
				laser_tf.translation,
				laser_size.hitbox.xy() * laser_scale,
				enemy_tf.translation,
				enemy_size.hitbox.xy() * enemy_tf.scale.xy(),
			);

			// perform collision
			if let Some(_) = collision {
				// remove the laser
				commands.entity(laser_entity).despawn();
				despawned_entities.insert(laser_entity);

				despawned_entities.insert(enemy_entity);
				enemy_hit_writer.send(EnemyHit {
					damage: 1,
					entity: enemy_entity,
					translation: enemy_tf.translation.clone(),
				});
				break;
			}
		}
	}
}

pub(super) fn enemy_laser_hit_player(
	mut commands: Commands,
	laser_query: Query<(Entity, &Transform, &Hitable), With<LaserFromEnemy>>,
	player_query: Query<(&Transform, &Hitable), With<Player>>,
	mut player_hit_writer: EventWriter<PlayerHit>,
) {
	if let Ok((player_tf, player_size)) = player_query.get_single() {
		let player_scale = player_tf.scale.xy();

		for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
			let laser_scale = laser_tf.scale.xy();

			// determine if collision
			let collision = collide(
				laser_tf.translation,
				laser_size.hitbox.xy() * laser_scale,
				player_tf.translation,
				player_size.hitbox.xy() * player_scale,
			);

			// perform the collision
			if let Some(_) = collision {
				// remove the laser
				commands.entity(laser_entity).despawn();

				player_hit_writer.send(PlayerHit {
					damage: 1,
					translation: player_tf.translation.clone(),
				});
				break;
			}
		}
	}
}
