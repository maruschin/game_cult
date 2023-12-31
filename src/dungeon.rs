//! Модуль предназначенный для генерации данжена

mod commands;
mod components;
mod enums;
mod level;

use crate::prelude::*;

use commands::{SpawnDoor, SpawnFloor, SpawnPlayer, SpawnWall};
use enums::TileType;
use level::Level;

use bevy::pbr::DirectionalLightShadowMap;
use std::f32::consts::PI;

use self::commands::SpawnEnemy;

pub const DUNGEON_ROW: usize = 15;
pub const DUNGEON_COLUMN: usize = 15;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 512 })
            .add_systems(Startup, setup)
            .add_systems(Update, gizmos_system);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Level {
        room_layer,
        wall_layer,
    } = Level::<DUNGEON_ROW, DUNGEON_COLUMN>::new();

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

    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // Ground
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: materials.add(Color::rgb(0.7, 0.7, 0.8).into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));

    if let Some(room) = room_layer.rooms.first() {
        let (i, j) = room.center();
        commands.add(SpawnPlayer::new((i * 4) as f32, 0.5, (j * 4) as f32));
    }

    for room in room_layer.rooms.iter() {
        let (i, j) = room.center();
        commands.add(SpawnEnemy::new((i * 4) as f32, 0.5, (j * 4) as f32));
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
