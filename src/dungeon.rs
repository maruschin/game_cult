//! Модуль предназначенный для генерации данжена

mod map;

use map::{new_map, TileType};

use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use std::f32::consts::PI;

use self::map::{DUNGEON_HEIGHT, DUNGEON_WIDE};

/// Просто плагин
pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 512 })
            .add_systems(Startup, setup)
            .add_systems(Update, (gizmos_system, rotate_camera));
    }
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    (
        (idx as i32) / (DUNGEON_WIDE as i32),
        (idx as i32) % (DUNGEON_WIDE as i32),
    )
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

    for (x, y, tile) in new_map().iter().enumerate().map(|(idx, tile)| {
        let (x, y) = idx_xy(idx);
        (x, y, tile)
    }) {
        match tile {
            | TileType::Wall => {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 4.0 })),
                    material: materials.add(Color::rgb(0., 0., 0.).into()),
                    transform: Transform::from_xyz((x * 4) as f32, 2.0, (y * 4) as f32),
                    ..default()
                });
            }
            | TileType::Floor => {
                commands.spawn(SceneBundle {
                    scene: asset_server.load("models/tileBrickB_medium.gltf.glb#Scene0"),
                    transform: Transform::from_xyz((x * 4) as f32, -1.0, (y * 4) as f32),
                    ..default()
                });
            }
        }
    }

    // barrel
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/barrel.gltf.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
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
    for x_idx in 0..DUNGEON_HEIGHT {
        for z_idx in 0..DUNGEON_WIDE {
            gizmos.cuboid(
                Transform::from_xyz((x_idx * 4) as f32, 2.0, (z_idx * 4) as f32)
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
