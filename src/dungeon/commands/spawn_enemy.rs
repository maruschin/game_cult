use crate::prelude::*;

use crate::dungeon::components::Enemy;

pub struct SpawnEnemy {
    pub position: Vec3,
}

impl SpawnEnemy {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3 { x, y, z },
        }
    }
}

impl Command for SpawnEnemy {
    fn apply(self, world: &mut World) {
        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            world.spawn((
                Enemy,
                RigidBody::Dynamic,
                Collider::cylinder(0.5, 0.4),
                SceneBundle {
                    scene: asset_server.load("models/barrel_large.glb#Scene0"),
                    transform: Transform::from_xyz(
                        self.position.x,
                        self.position.y,
                        self.position.z,
                    ),
                    ..default()
                },
            ));
        }
    }
}
