use bevy::prelude::*;

pub(super) const EXPLOSION_DURATION: usize = 16;

pub(super) struct ExplosionAssets {
	pub explosion_atlas: Handle<TextureAtlas>,
	pub enemy_explosion_sound: Handle<AudioSource>,
	pub player_explosion_sound: Handle<AudioSource>,
}

pub(super) fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let texture = asset_server.load("explo_a_sheet.png");
	let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(64., 64.), 4, 4);
	let explosion = texture_atlases.add(texture_atlas);
	commands.insert_resource(ExplosionAssets {
		explosion_atlas: explosion,
		enemy_explosion_sound: asset_server.load("mixkit-arcade-chiptune-explosion-1691.flac"),
		player_explosion_sound: asset_server.load("mixkit-funny-system-break-down-2955.flac"),
	})
}
