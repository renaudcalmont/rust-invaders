use bevy::prelude::*;
use std::f32::consts::PI;

use super::components::*;
use super::events::*;
use super::resources::*;
use crate::world::components::*;
use crate::world::*;

pub(super) fn spawn_enemy(
	mut commands: Commands,
	assets: Res<EnemyAssets>,
	mut enemy_count: ResMut<EnemyCount>,
	mut formation_maker: ResMut<FormationMaker>,
	mut windows: ResMut<Windows>,
) {
	if enemy_count.0 < super::ENEMY_MAX_COUNT {
		let window = windows.get_primary_mut().unwrap();

		// get formation and start x/y
		let formation = formation_maker.make(window.width(), window.height());
		let (x, y) = formation.start;

		commands
			.spawn_bundle(SpriteBundle {
				texture: assets.enemy_image.clone(),
				transform: Transform {
					translation: Vec3::new(x, y, 10.),
					scale: SPRITE_SCALE,
					..Default::default()
				},
				..Default::default()
			})
			.insert(Name::new("Enemy"))
			.insert(Enemy)
			.insert(formation)
			.insert(Hitable { hitbox: ENEMY_SIZE })
			.insert(super::ENEMY_HEALTH);

		enemy_count.0 += 1;
	}
}

pub(super) fn enemy_fire(
	mut commands: Commands,
	assets: Res<EnemyAssets>,
	audio: Res<Audio>,
	enemy_query: Query<&Transform, With<Enemy>>,
) {
	for &tf in enemy_query.iter() {
		let (x, y) = (tf.translation.x, tf.translation.y);
		// spawn enemy laser sprite
		commands
			.spawn_bundle(SpriteBundle {
				texture: assets.laser_image.clone(),
				transform: Transform {
					translation: Vec3::new(x, y - 15., 0.),
					rotation: Quat::from_rotation_x(PI),
					scale: SPRITE_SCALE,
					..Default::default()
				},
				..Default::default()
			})
			.insert(LaserFromEnemy)
			.insert(Hitable { hitbox: LASER_SIZE })
			.insert(Movable { auto_despawn: true })
			.insert(Velocity {
				translation: Vec3::new(0., -1., 0.),
				..Default::default()
			});
		audio.play(assets.laser_sound.clone());
	}
}

pub(super) fn enemy_move(mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>) {
	for (mut transform, mut formation) in query.iter_mut() {
		// current position
		let (x_org, y_org) = (transform.translation.x, transform.translation.y);

		// max distance
		let max_distance = TIME_STEP * formation.speed;

		// 1 for counter clockwise, -1 clockwise
		let dir: f32 = if formation.start.0 < 0. { 1. } else { -1. };
		let (x_pivot, y_pivot) = formation.pivot;
		let (x_radius, y_radius) = formation.radius;

		// compute next angle (based on time for now)
		let angle = formation.angle
			+ dir * formation.speed * TIME_STEP / (x_radius.min(y_radius) * PI / 2.);

		// compute target x/y
		let x_dst = x_radius * angle.cos() + x_pivot;
		let y_dst = y_radius * angle.sin() + y_pivot;

		// compute distance
		let dx = x_org - x_dst;
		let dy = y_org - y_dst;
		let distance = (dx * dx + dy * dy).sqrt();
		let distance_ratio = if distance != 0. { max_distance / distance } else { 0. };

		// compute final x/y
		let x = x_org - dx * distance_ratio;
		let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
		let y = y_org - dy * distance_ratio;
		let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

		// start rotating the formation angle only when sprite is on or close to ellipse
		if distance < max_distance * formation.speed / 20. {
			formation.angle = angle;
		}

		let translation = &mut transform.translation;
		(translation.x, translation.y) = (x, y);
	}
}

pub(super) fn enemy_hit_listener(
	mut commands: Commands,
	mut enemy_query: Query<&mut Health, With<Enemy>>,
	mut enemy_hit_reader: EventReader<EnemyHit>,
	mut enemy_killed_writer: EventWriter<EnemyKilled>,
	mut enemy_count: ResMut<EnemyCount>,
) {
	for event in enemy_hit_reader.iter() {
		if let Ok(mut enemy_health) = enemy_query.get_mut(event.entity) {
			enemy_health.level -= event.damage;
			if enemy_health.level <= 0 {
				commands.entity(event.entity).despawn();
				enemy_killed_writer.send(EnemyKilled {
					translation: event.translation.clone(),
				});

				enemy_count.0 -= 1;
				return;
			}

			commands.entity(event.entity).insert(Highlight::default());
		}
	}
}

pub(super) fn highlight_animation(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut Highlight, &mut Sprite), With<Enemy>>,
) {
	for (entity, mut highlight, mut sprite) in query.iter_mut() {
		highlight.timer.tick(time.delta());
		sprite.color.set_g(0.);
		sprite.color.set_b(0.);
		if highlight.timer.finished() {
			commands.entity(entity).remove::<Highlight>();
			sprite.color.set_g(1.);
			sprite.color.set_b(1.);
		}
	}
}
