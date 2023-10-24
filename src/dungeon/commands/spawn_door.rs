use bevy::ecs::system::Command;
use bevy::prelude::*;

use crate::dungeon::enums::DoorType;

pub struct SpawnDoor {
    pub position: Vec3,
    pub door_type: DoorType,
}

impl SpawnDoor {
    pub fn new(x: f32, y: f32, z: f32, door_type: DoorType) -> Self {
        Self {
            position: Vec3 { x, y, z },
            door_type,
        }
    }
}

impl Command for SpawnDoor {
    fn apply(self, world: &mut World) {
        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            let asset_path = "models/wall_gateDoor.gltf.glb#Scene0";
            let Vec3 { x, y, z } = self.position;
            let mut batch = vec![SceneBundle {
                scene: asset_server.load("models/tileBrickB_medium.gltf.glb#Scene0"),
                transform: Transform::from_xyz(x, y - 1.0, z),
                ..default()
            }];

            match self.door_type {
                | DoorType::Bottom => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(asset_path),
                        transform: Transform::from_xyz(x, y, z + 2.0)
                            .with_rotation(Quat::from_rotation_y((0.0 as f32).to_radians())),
                        ..default()
                    });
                }
                | DoorType::Right => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(asset_path),
                        transform: Transform::from_xyz(x + 2.0, y, z)
                            .with_rotation(Quat::from_rotation_y((90.0 as f32).to_radians())),
                        ..default()
                    });
                }
                | DoorType::Top => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(asset_path),
                        transform: Transform::from_xyz(x, y, z - 2.0)
                            .with_rotation(Quat::from_rotation_y((0.0 as f32).to_radians())),
                        ..default()
                    });
                }
                | DoorType::Left => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(asset_path),
                        transform: Transform::from_xyz(x - 2.0, y, z)
                            .with_rotation(Quat::from_rotation_y((90.0 as f32).to_radians())),
                        ..default()
                    });
                }
            }

            world.spawn_batch(batch);
        }
    }
}
