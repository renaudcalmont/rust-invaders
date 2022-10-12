use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

pub(crate) struct InspectorPlugin;

impl Plugin for InspectorPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(WorldInspectorPlugin::new())
			.register_inspectable::<crate::world::components::Health>()
			.register_inspectable::<crate::world::components::Hitable>()
			.register_inspectable::<crate::world::components::Movable>()
			.register_inspectable::<crate::world::components::Velocity>();
	}
}
