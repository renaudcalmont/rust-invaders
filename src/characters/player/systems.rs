use bevy::prelude::*;

use super::components::*;
use super::events::*;
use super::resources::*;
use crate::world::components::*;
use crate::world::SPRITE_SCALE;

pub fn spawn_camera(mut commands: Commands) {
	commands.spawn_bundle(Camera2dBundle::default()).insert(Name::new("Camera"));
}

pub(super) fn spawn_player(
	mut commands: Commands,
	mut player_state: ResMut<PlayerState>,
	assets: Res<PlayerAssets>,
	mut windows: ResMut<Windows>,
	time: Res<Time>,
) {
	let now = time.seconds_since_startup();
	let last_shot = player_state.last_shot;

	if !player_state.alive
		&& (last_shot == -1. || now > last_shot + super::PLAYER_RESPAWN_DELAY)
	{
		let window = windows.get_primary_mut().unwrap();
		let bottom = -window.height() / 2.;
		commands
			.spawn_bundle(SpriteBundle {
				texture: assets.player_image.clone(),
				transform: Transform {
					translation: Vec3::new(0., bottom + PLAYER_SIZE.y * SPRITE_SCALE.y, 10.),
					scale: SPRITE_SCALE,
					..Default::default()
				},
				..Default::default()
			})
			.insert(Name::new("Player"))
			.insert(Player)
			.insert(Hitable { hitbox: PLAYER_SIZE })
			.insert(Health::default())
			.insert(Velocity::default())
			.insert(Movable::default());
		player_state.spawned();
	}
}

pub(super) fn player_move(
	kb: Res<Input<KeyCode>>,
	mut query: Query<&mut Velocity, With<Player>>,
) {
	let mut velocity = match query.get_single_mut() {
		Ok(it) => it,
		_ => return,
	};
	velocity.translation.x = if kb.pressed(KeyCode::Left) {
		-1.
	} else if kb.pressed(KeyCode::Right) {
		1.
	} else {
		0.
	}
}

pub(super) fn player_fire(
	mut commands: Commands,
	kb: Res<Input<KeyCode>>,
	assets: Res<PlayerAssets>,
	audio: Res<Audio>,
	query: Query<&Transform, With<Player>>,
) {
	if let Ok(player_tf) = query.get_single() {
		if kb.just_pressed(KeyCode::Space) {
			let (x, y) = (player_tf.translation.x, player_tf.translation.y);
			let x_offset = PLAYER_SIZE.x / 2. * SPRITE_SCALE.x - 5.;

			let mut spawn_laser = |x_offset: f32| {
				commands
					.spawn_bundle(SpriteBundle {
						texture: assets.laser_image.clone(),
						transform: Transform {
							translation: Vec3::new(x + x_offset, y + 15., 0.),
							scale: SPRITE_SCALE,
							..Default::default()
						},
						..Default::default()
					})
					.insert(LaserFromPlayer)
					.insert(Hitable { hitbox: LASER_SIZE })
					.insert(Movable { auto_despawn: true })
					.insert(Velocity {
						translation: Vec3::new(0., 1., 0.),
						..Default::default()
					});
			};

			spawn_laser(x_offset);
			spawn_laser(-x_offset);
			audio.play(assets.laser_sound.clone());
		}
	}
}

pub(super) fn player_hit_listener(
	mut commands: Commands,
	time: Res<Time>,
	mut player_query: Query<(Entity, &mut Health), With<Player>>,
	mut player_hit_reader: EventReader<PlayerHit>,
	mut player_killed_writer: EventWriter<PlayerKilled>,
	mut player_state: ResMut<PlayerState>,
) {
	if let Ok((player_entity, mut player_health)) = player_query.get_single_mut() {
		for event in player_hit_reader.iter() {
			player_health.level -= event.damage;
			if player_health.level <= 0 {
				commands.entity(player_entity).despawn();
				player_killed_writer.send(PlayerKilled {
					translation: event.translation.clone(),
				});

				player_state.shot(time.seconds_since_startup());
				return;
			}
		}
	}
}
