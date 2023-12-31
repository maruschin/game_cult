use crate::prelude::*;

use crate::dungeon::components::{Player, PlayerCamera};
use bevy_xpbd_3d::math::{Scalar, Vector};

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
            world
                .spawn((
                    Player,
                    CharacterControllerBundle::new(
                        Collider::capsule(0.25, 0.5),
                        Vector::NEG_Y * 9.81 * 2.0,
                    )
                    .with_movement(
                        500.0,
                        0.1,
                        30.0,
                        0.1,
                        7.0,
                        (30.0 as Scalar).to_radians(),
                    ),
                    SceneBundle {
                        scene: asset_server.load("models/characters/barbarian.glb#Scene0"),
                        transform: Transform::from_xyz(
                            self.position.x,
                            self.position.y,
                            self.position.z,
                        ),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        PlayerCamera,
                        LookTransformBundle {
                            transform: LookTransform::new(
                                Vec3::new(2.0, 5.0, 5.0),
                                Vec3::ZERO,
                                Vec3::Y,
                            ),
                            smoother: Smoother::new(0.9), // Value between 0.0 and 1.0, higher is smoother.
                        },
                        Camera3dBundle {
                            projection: PerspectiveProjection {
                                fov: 75.0_f32.to_radians(),
                                ..default()
                            }
                            .into(),
                            ..default()
                        },
                    ));
                });
        }
    }
}
