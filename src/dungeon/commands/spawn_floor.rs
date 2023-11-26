use super::super::enums::FloorType;
use crate::prelude::*;

pub struct SpawnFloor {
    pub position: Vec3,
    floor_type: FloorType,
}

impl SpawnFloor {
    pub fn new(x: f32, y: f32, z: f32, floor_type: FloorType) -> Self {
        Self {
            position: Vec3 { x, y, z },
            floor_type,
        }
    }
}

fn floor_model(floor_type: FloorType) -> Option<String> {
    match floor_type {
        | FloorType::Empthy => None,
        | FloorType::Room => Some("models/tileBrickB_medium.gltf.glb#Scene0".to_string()),
        | FloorType::Path => Some("models/tileBrickA_medium.gltf.glb#Scene0".to_string()),
    }
}

impl Command for SpawnFloor {
    fn apply(self, world: &mut World) {
        if let Some(asset_server) = world.get_resource::<AssetServer>() {
            match self.floor_type {
                | floor_type @ FloorType::Room | floor_type @ FloorType::Path => {
                    world.spawn((
                        SceneBundle {
                            scene: asset_server.load(floor_model(floor_type).unwrap()),
                            transform: Transform::from_xyz(
                                self.position.x,
                                self.position.y - 1.0,
                                self.position.z,
                            ),
                            ..default()
                        },
                        Collider::cuboid(2.0, 1.0, 2.0),
                    ));
                }
                | FloorType::Empthy => {
                    let mesh_handle =
                        world.resource_scope(|_world, mut meshes: Mut<Assets<Mesh>>| {
                            meshes.add(Mesh::from(shape::Cube { size: 4.0 }))
                        });

                    let material_handle = world.resource_scope(
                        |_world, mut materials: Mut<Assets<StandardMaterial>>| {
                            materials.add(Color::rgb(0., 0., 0.).into())
                        },
                    );

                    world.spawn((
                        PbrBundle {
                            mesh: mesh_handle,
                            material: material_handle,
                            transform: Transform::from_xyz(self.position.x, 2.0, self.position.z),
                            ..default()
                        },
                        Collider::cuboid(2.0, 2.0, 2.0),
                    ));
                }
            }
        }
    }
}
