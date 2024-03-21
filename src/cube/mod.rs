use bevy::prelude::*;

mod systems;
mod components;

pub struct CubePlugin;

impl Plugin for CubePlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Startup, systems::setup);
      app.add_systems(Update, systems::system);
  }
}
