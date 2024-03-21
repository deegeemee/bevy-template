use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

mod systems;
mod components;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
  fn build(&self, app: &mut App) {
      app
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, systems::setup)
        .add_systems(Update, systems::system);
  }
}
