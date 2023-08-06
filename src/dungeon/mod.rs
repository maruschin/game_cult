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
            .add_systems(Update, rotate_camera);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 6.0, 24.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // floor
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/floorDecoration_tilesLarge.gltf.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

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
            illuminance: 20000.,
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

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
}
