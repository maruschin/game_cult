mod dungeon;
mod prelude {
    pub use bevy::prelude::*;
}

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
        .add_plugins(DungeonPlugin)
        .run();
}
