use bevy::prelude::*;
use bevy::window::*;

mod camera;
mod cube;
mod fps;
mod plane;
mod pointlight;

use camera::CameraPlugin;
use cube::CubePlugin;
use fps::FpsPlugin;
use plane::PlanePlugin;
use pointlight::PointlightPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin { ..default() })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy playground!".to_string(),
                        resolution: WindowResolution::new(800., 800. / 16. * 10.),
                        position: WindowPosition::Centered(MonitorSelection::Current),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
                    close_when_requested: true,
                }),
        )
        .add_plugins((
            FpsPlugin,
            PlanePlugin,
            CubePlugin,
            CameraPlugin,
            PointlightPlugin,
        ))
        .run();
}
