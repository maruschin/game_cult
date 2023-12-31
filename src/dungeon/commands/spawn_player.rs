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
            world.spawn((
                Player,
                CharacterControllerBundle::new(
                    Collider::capsule(0.25, 0.5),
                    Vector::NEG_Y * 9.81 * 2.0,
                )
                .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
                SceneBundle {
                    scene: asset_server.load("models/characters/barbarian.glb#Scene0"),
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
