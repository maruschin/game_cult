//! Модуль предназначенный для генерации данжена

use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use std::f32::consts::PI;

/// Просто плагин
pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 512 })
            .add_systems(Startup, setup)
            .add_systems(Update, (gizmos_system, rotate_camera));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 18.0, 24.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // floor
    for x_idx in -3..4 {
        for z_idx in -1..2 {
            commands.spawn(SceneBundle {
                scene: asset_server.load("models/tileBrickB_medium.gltf.glb#Scene0"),
                transform: Transform::from_xyz((x_idx * 4) as f32, -1.0, (z_idx * 4) as f32),
                ..default()
            });
        }
    }

    commands.spawn(SceneBundle {
        scene: asset_server.load("models/wall_door.gltf.glb#Scene0"),
        transform: Transform::from_xyz(-9.0, 0.0, 2.0),
        ..default()
    });
    for x_idx in -1..2 {
        commands.spawn(SceneBundle {
            scene: asset_server.load("models/wall_gate.gltf.glb#Scene0"),
            transform: Transform::from_xyz((x_idx * 4) as f32, 0.0, 2.0),
            ..default()
        });
    }
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/wall_end.gltf.glb#Scene0"),
        transform: Transform::from_xyz(6.0, 0.0, 2.0),
        ..default()
    });

    for x_idx in -3..4 {
        commands.spawn(SceneBundle {
            scene: asset_server.load("models/wall.gltf.glb#Scene0"),
            transform: Transform::from_xyz((x_idx * 4) as f32, 0.0, -6.0),
            ..default()
        });

        commands.spawn(SceneBundle {
            scene: asset_server.load("models/wall.gltf.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new((x_idx * 4) as f32, 0.0, 6.0),
                rotation: Quat::from_rotation_y(PI),
                ..default()
            },
            ..default()
        });
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
    for x_idx in -4..5 {
        for z_idx in -4..5 {
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
        camera_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time_delta_rotation));
    }
    if keys.pressed(KeyCode::E) {
        camera_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-time_delta_rotation));
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
