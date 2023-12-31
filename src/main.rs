mod camera;
mod character;
mod dungeon;
mod main_menu;
mod prelude {
    pub use super::character::*;
    pub use bevy::ecs::system::Command;
    pub use bevy::prelude::*;
    pub use bevy_xpbd_3d::prelude::*;
}

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use character::CharacterControllerPlugin;
use dungeon::DungeonPlugin;
//use main_menu::MainMenuPlugin;
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
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(CharacterControllerPlugin)
        .add_plugins(DungeonPlugin)
        //.add_plugins(MainMenuPlugin)
        .run();
}
