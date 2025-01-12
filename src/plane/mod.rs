use bevy::prelude::*;

mod components;
mod systems;

pub struct PlanePlugin;

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup)
            .add_systems(Update, systems::system);
    }
}
