use bevy::prelude::*;

mod systems;
mod components;

pub struct PlanePlugin;

impl Plugin for PlanePlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Startup, systems::setup);
  }
}
