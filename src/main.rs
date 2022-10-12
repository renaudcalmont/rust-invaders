use bevy::prelude::*;

mod characters;
mod interactions;
mod world;

mod inspector;

fn main() {
	App::new()
		.insert_resource(background_color())
		.insert_resource(window())
		.add_plugins(DefaultPlugins)
		////.add_plugin(inspector::InspectorPlugin)
		.add_plugin(world::WorldPlugin)
		.add_plugin(characters::enemy::EnemyPlugin)
		.add_plugin(characters::player::PlayerPlugin)
		.add_plugin(interactions::explosion::ExplosionPlugin)
		.add_plugin(interactions::laser_hit::LaserHitPlugin)
		.run()
}

fn background_color() -> ClearColor {
	ClearColor(Color::rgb(0.04, 0.04, 0.04))
}

fn window() -> WindowDescriptor {
	WindowDescriptor {
		title: "Rust Invaders!".to_string(),
		width: 598.0,
		height: 676.0,
		resizable: false,
		..Default::default()
	}
}
