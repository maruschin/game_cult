use crate::prelude::*;

use crate::dungeon::components::{Player, PlayerCamera};

pub struct SpawnPlayer {
    pub position: Vec3,
}

impl SpawnPlayer {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3 { x, y, z },
        }
    }
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            world.spawn((
                Player,
                RigidBody::Fixed,
                Collider::capsule_y(0.0, 0.5),
                SceneBundle {
                    scene: asset_server.load("models/Characters/character_barbarian.gltf#Scene0"),
                    transform: Transform::from_xyz(
                        self.position.x,
                        self.position.y,
                        self.position.z,
                    ),
                    ..default()
                },
            ));

            world.spawn((
                PlayerCamera,
                Camera3dBundle {
                    projection: PerspectiveProjection {
                        fov: 75.0_f32.to_radians(),
                        ..default()
                    }
                    .into(),
                    ..default()
                },
            ));
        }
    }
}
