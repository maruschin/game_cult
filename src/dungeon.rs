//! Модуль предназначенный для генерации данжена

mod commands;
mod components;
mod enums;
mod map;

use commands::{SpawnDoor, SpawnFloor, SpawnPlayer, SpawnWall};
use components::Player;
use enums::TileType;
use map::Map;

use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use std::f32::consts::PI;

pub const DUNGEON_ROW: usize = 15;
pub const DUNGEON_COLUMN: usize = 15;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 512 })
            .add_systems(Startup, setup)
            .add_systems(Update, (gizmos_system, move_player, camera_following_player));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let Map {
        room_layer,
        wall_layer,
    } = Map::<DUNGEON_ROW, DUNGEON_COLUMN>::new();

    for (x, z, tile) in room_layer.layer.iter() {
        commands.add(SpawnFloor::new(x, 0.0, z, *tile));
    }

    for (x, z, tile) in wall_layer.layer.iter() {
        match tile {
            | TileType::Empthy => {}
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

fn camera_following_player(
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
    mut player_transform_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = camera_transform_query.single_mut();
    let player_transform = player_transform_query.single_mut();

    camera_transform.translation =
        player_transform.translation + player_transform.back() * 4.0 + player_transform.up() * 1.5;
    camera_transform.rotation = player_transform.rotation;
}

fn move_player(
    mut player_transform_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = player_transform_query.single_mut();

    let time_delta_rotation = time.delta_seconds() * 2.0;
    let time_delta_move = time_delta_rotation * 4.0;

    if keys.pressed(KeyCode::Q) {
        player_transform.rotate(Quat::from_rotation_y(time_delta_rotation));
    }
    if keys.pressed(KeyCode::E) {
        player_transform.rotate(Quat::from_rotation_y(-time_delta_rotation));
    }

    if keys.pressed(KeyCode::W) {
        let forward_vector = {
            let mut forward_vector = player_transform.forward();
            forward_vector.y = 0.0;
            forward_vector.normalize() * time_delta_move
        };
        player_transform.translation += forward_vector;
    }
    if keys.pressed(KeyCode::S) {
        let back_vector = {
            let mut back_vector = player_transform.back();
            back_vector.y = 0.0;
            back_vector.normalize() * time_delta_move
        };
        player_transform.translation += back_vector;
    }
    if keys.pressed(KeyCode::A) {
        let left_vector = player_transform.left() * time_delta_move;
        player_transform.translation += left_vector;
    }
    if keys.pressed(KeyCode::D) {
        let right_vector = player_transform.right() * time_delta_move;
        player_transform.translation += right_vector;
    }
}
