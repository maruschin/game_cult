use crate::dungeon::map::CornerType;
use bevy::ecs::system::Command;
use bevy::prelude::*;

pub struct SpawnCorner {
    pub position: Vec3,
    pub corner_type: CornerType,
}

impl SpawnCorner {
    pub fn new(x: f32, y: f32, z: f32, corner_type: CornerType) -> Self {
        Self {
            position: Vec3 { x, y, z },
            corner_type,
        }
    }
}

impl Command for SpawnCorner {
    fn apply(self, world: &mut World) {
        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            let wall_asset_path = "models/wallSingle.gltf.glb#Scene0";
            let Vec3 { x, y, z } = self.position;
            let mut batch = vec![SceneBundle {
                scene: asset_server.load("models/tileBrickB_medium.gltf.glb#Scene0"),
                transform: Transform::from_xyz(x, y - 1.0, z),
                ..default()
            }];

            match self.corner_type {
                | CornerType::TopLeft => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
                        transform: Transform::from_xyz(x, y, z - 2.0)
                            .with_rotation(Quat::from_rotation_y((0.0 as f32).to_radians())),
                        ..default()
                    });
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
                        transform: Transform::from_xyz(x + 2.0, y, z)
                            .with_rotation(Quat::from_rotation_y((270.0 as f32).to_radians())),
                        ..default()
                    });
                }
                | CornerType::TopRight => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
                        transform: Transform::from_xyz(x, y, z + 2.0)
                            .with_rotation(Quat::from_rotation_y((180.0 as f32).to_radians())),
                        ..default()
                    });
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
                        transform: Transform::from_xyz(x + 2.0, y, z)
                            .with_rotation(Quat::from_rotation_y((270.0 as f32).to_radians())),
                        ..default()
                    });
                }
                | CornerType::BottomLeft => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
                        transform: Transform::from_xyz(x, y, z - 2.0)
                            .with_rotation(Quat::from_rotation_y((0.0 as f32).to_radians())),
                        ..default()
                    });
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
                        transform: Transform::from_xyz(x - 2.0, y, z)
                            .with_rotation(Quat::from_rotation_y((90.0 as f32).to_radians())),
                        ..default()
                    });
                }
                | CornerType::BottomRight => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
                        transform: Transform::from_xyz(x, y, z + 2.0)
                            .with_rotation(Quat::from_rotation_y((180.0 as f32).to_radians())),
                        ..default()
                    });
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
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
