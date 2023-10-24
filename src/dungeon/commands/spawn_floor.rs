use bevy::ecs::system::Command;
use bevy::prelude::*;

pub struct SpawnFloor {
    pub position: Vec3,
}

impl SpawnFloor {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3 { x, y, z },
        }
    }
}

impl Command for SpawnFloor {
    fn apply(self, world: &mut World) {
        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            world.spawn(SceneBundle {
                scene: asset_server.load("models/tileBrickB_medium.gltf.glb#Scene0"),
                transform: Transform::from_xyz(
                    self.position.x,
                    self.position.y - 1.0,
                    self.position.z,
                ),
                ..default()
            });
        }
    }
}
