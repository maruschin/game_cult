use crate::dungeon::enums::{CornerType, WallType};
use bevy::ecs::system::Command;
use bevy::prelude::*;

pub struct SpawnWall {
    pub position: Vec3,
    pub wall_type: WallType,
}

impl SpawnWall {
    pub fn new(x: f32, y: f32, z: f32, wall_type: WallType) -> Self {
        Self {
            position: Vec3 { x, y, z },
            wall_type,
        }
    }
}

impl Command for SpawnWall {
    fn apply(self, world: &mut World) {
        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            let wall_asset_path = "models/wall/wall.glb#Scene0";
            let Vec3 { x, y, z } = self.position;
            let mut batch: Vec<SceneBundle> = vec![];

            match self.wall_type {
                | WallType::Bottom => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
                        transform: Transform::from_xyz(x - 2.0, y, z)
                            .with_rotation(Quat::from_rotation_y((90.0 as f32).to_radians())),
                        ..default()
                    });
                }
                | WallType::Right => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
                        transform: Transform::from_xyz(x, y, z + 2.0)
                            .with_rotation(Quat::from_rotation_y((180.0 as f32).to_radians())),
                        ..default()
                    });
                }
                | WallType::Top => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
                        transform: Transform::from_xyz(x + 2.0, y, z)
                            .with_rotation(Quat::from_rotation_y((270.0 as f32).to_radians())),
                        ..default()
                    });
                }
                | WallType::Left => {
                    batch.push(SceneBundle {
                        scene: asset_server.load(wall_asset_path),
                        transform: Transform::from_xyz(x, y, z - 2.0)
                            .with_rotation(Quat::from_rotation_y((0.0 as f32).to_radians())),
                        ..default()
                    });
                }
                | WallType::InternalCorner(corner_type) => {
                    // Top wall
                    if corner_type == CornerType::TopLeft || corner_type == CornerType::TopRight {
                        batch.push(SceneBundle {
                            scene: asset_server.load(wall_asset_path),
                            transform: Transform::from_xyz(x + 2.0, y, z)
                                .with_rotation(Quat::from_rotation_y((270.0 as f32).to_radians())),
                            ..default()
                        })
                    }
                    // Left wall
                    if corner_type == CornerType::TopLeft || corner_type == CornerType::BottomLeft {
                        batch.push(SceneBundle {
                            scene: asset_server.load(wall_asset_path),
                            transform: Transform::from_xyz(x, y, z - 2.0)
                                .with_rotation(Quat::from_rotation_y((0.0 as f32).to_radians())),
                            ..default()
                        })
                    };
                    // Right wall
                    if corner_type == CornerType::TopRight || corner_type == CornerType::BottomRight
                    {
                        batch.push(SceneBundle {
                            scene: asset_server.load(wall_asset_path),
                            transform: Transform::from_xyz(x, y, z + 2.0)
                                .with_rotation(Quat::from_rotation_y((180.0 as f32).to_radians())),
                            ..default()
                        });
                    };
                    // Bottom wall
                    if corner_type == CornerType::BottomLeft
                        || corner_type == CornerType::BottomRight
                    {
                        batch.push(SceneBundle {
                            scene: asset_server.load(wall_asset_path),
                            transform: Transform::from_xyz(x - 2.0, y, z)
                                .with_rotation(Quat::from_rotation_y((90.0 as f32).to_radians())),
                            ..default()
                        });
                    }
                }
            }

            world.spawn_batch(batch);
        }
    }
}
