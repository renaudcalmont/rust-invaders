use bevy::prelude::*;

use super::{components::*, resources::*};
use crate::characters::{enemy::events::*, player::events::*};

pub(super) fn enemy_killed_listener(
	mut commands: Commands,
	assets: Res<ExplosionAssets>,
	audio: Res<Audio>,
	mut enemy_killed_reader: EventReader<EnemyKilled>,
) {
	for event in enemy_killed_reader.iter() {
		commands
			.spawn_bundle(SpriteSheetBundle {
				texture_atlas: assets.explosion_atlas.clone(),
				transform: Transform {
					translation: event.translation,
					..Default::default()
				},
				..Default::default()
			})
			.insert(Explosion::default());
		audio.play(assets.enemy_explosion_sound.clone());
	}
}

pub(super) fn player_killed_listener(
	mut commands: Commands,
	assets: Res<ExplosionAssets>,
	audio: Res<Audio>,
	mut player_killed_reader: EventReader<PlayerKilled>,
) {
	for event in player_killed_reader.iter() {
		commands
			.spawn_bundle(SpriteSheetBundle {
				texture_atlas: assets.explosion_atlas.clone(),
				transform: Transform {
					translation: event.translation,
					..Default::default()
				},
				..Default::default()
			})
			.insert(Explosion::default());
		audio.play(assets.player_explosion_sound.clone());
	}
}

pub(super) fn explosion_animation(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut Explosion, &mut TextureAtlasSprite), With<Explosion>>,
) {
	for (entity, mut explosion, mut sprite) in query.iter_mut() {
		explosion.timer.tick(time.delta());
		if explosion.timer.finished() {
			sprite.index += 1; // move to next sprite cell
			if sprite.index >= EXPLOSION_DURATION {
				commands.entity(entity).despawn()
			}
		}
	}
}
