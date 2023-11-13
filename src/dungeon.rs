//! Модуль предназначенный для генерации данжена

mod commands;
mod enums;
mod map;

use commands::{SpawnDoor, SpawnFloor, SpawnPlayer, SpawnWall};
pub use enums::{TileType, WallType};
use map::Map;

use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use std::f32::consts::PI;

pub const DUNGEON_ROW: usize = 15;
pub const DUNGEON_COLUMN: usize = 15;

/// Просто плагин
pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 512 })
            .add_systems(Startup, setup)
            .add_systems(Update, (gizmos_system, rotate_camera));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 18.0, 24.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let Map { room_layer } = Map::<DUNGEON_ROW, DUNGEON_COLUMN>::new();
    for (x, z, tile) in room_layer.layer.iter() {
        match tile {
            | TileType::Empthy => {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 4.0 })),
                    material: materials.add(Color::rgb(0., 0., 0.).into()),
                    transform: Transform::from_xyz(x, 2.0, z),
                    ..default()
                });
            }
            | TileType::Path => {
                commands.spawn(SceneBundle {
                    scene: asset_server.load("models/tileBrickA_medium.gltf.glb#Scene0"),
                    transform: Transform::from_xyz(x, -1.0, z),
                    ..default()
                });
            }
            | TileType::Floor => {
                commands.add(SpawnFloor::new(x, 0.0, z));
            }
            | TileType::Wall(wall_type) => {
                commands.add(SpawnWall::new(x, 0.0, z, *wall_type));
            }
            | TileType::Door(door_type) => {
                commands.add(SpawnDoor::new(x, 0.0, z, *door_type));
            }
        }
    }

    // Spawn player
    if let Some(room) = room_layer.rooms.first() {
        let (i, j) = room.center();
        commands.add(SpawnPlayer::new((i * 4) as f32, 0.5, (j * 4) as f32));
    }

    // barrel
    for room in room_layer.rooms.iter() {
        let (i, j) = room.center();
        commands.spawn(SceneBundle {
            scene: asset_server.load("models/barrel.gltf.glb#Scene0"),
            transform: Transform::from_xyz((i * 4) as f32, 0.5, (j * 4) as f32),
            ..default()
        });
    }
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 15000.,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}

fn gizmos_system(mut gizmos: Gizmos) {
    for i in 0..DUNGEON_ROW {
        for j in 0..DUNGEON_COLUMN {
            gizmos.cuboid(
                Transform::from_xyz((i * 4) as f32, 2.0, (j * 4) as f32)
                    .with_scale(Vec3::splat(4.)),
                Color::BLACK,
            );
        }
    }
}

fn rotate_camera(
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_transform_query.single_mut();
    let time_delta_rotation = time.delta_seconds() / 2.0;
    let time_delta_move = time_delta_rotation * 16.0;

    if keys.pressed(KeyCode::Q) {
        camera_transform.rotate(Quat::from_rotation_y(time_delta_rotation));
    }
    if keys.pressed(KeyCode::E) {
        camera_transform.rotate(Quat::from_rotation_y(-time_delta_rotation));
    }

    if keys.pressed(KeyCode::W) {
        let mut forward_vector = camera_transform.forward();
        forward_vector.y = 0.0;
        camera_transform.translation += forward_vector.normalize() * time_delta_move;
    }
    if keys.pressed(KeyCode::S) {
        let mut back_vector = camera_transform.back();
        back_vector.y = 0.0;
        camera_transform.translation += back_vector.normalize() * time_delta_move;
    }
    if keys.pressed(KeyCode::A) {
        camera_transform.translation =
            camera_transform.translation + camera_transform.left() * time_delta_move;
    }
    if keys.pressed(KeyCode::D) {
        camera_transform.translation =
            camera_transform.translation + camera_transform.right() * time_delta_move;
    }
}
