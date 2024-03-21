use bevy::prelude::*;

mod systems;
mod components;

pub struct PointlightPlugin;

impl Plugin for PointlightPlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Startup, systems::setup);
  }
}
