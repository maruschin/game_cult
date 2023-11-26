mod dungeon;
mod prelude {
    pub use bevy::ecs::system::Command;
    pub use bevy::prelude::*;
    pub use bevy_rapier3d::prelude::*;
}

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use dungeon::DungeonPlugin;
use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cult of meal: the story about talking food".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(DungeonPlugin)
        .run();
}
