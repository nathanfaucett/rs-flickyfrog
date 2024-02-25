use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;

use crate::{
    app::{GameState, INV_WORLD_SIZE},
    game::{CollisionLayer, Piller},
};

#[derive(Resource, Deref, DerefMut)]
pub struct PlayerCameraOffset(Vec2);

#[derive(Default, Component)]
pub struct Player {
    pub tongue_out: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerCameraOffset(Vec2::new(0.0, 0.0)))
            .add_systems(
                PreUpdate,
                sync_player_camera_offset.run_if(in_state(GameState::Game)),
            )
            .add_systems(
                PostUpdate,
                (camera_follow_player).run_if(in_state(GameState::Game)),
            )
            .add_systems(Last, (player_controller).run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct TonguePart;

#[derive(Component)]
pub struct TongueJoint;

pub fn player_controller(
    mut commands: Commands,
    mut player_q: Query<(Entity, &mut Player, &GlobalTransform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    pillers_q: Query<(Entity, &GlobalTransform, &Collider), With<Piller>>,
    tongue_joints_q: Query<Entity, With<TongueJoint>>,
    tongue_q: Query<Entity, With<TonguePart>>,
    touches: Res<Touches>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let mut just_pressed = false;
    let mut just_released = false;
    let mut left = false;

    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
            just_pressed = true;
            let window = q_windows.single();
            left = finger.position().x < window.width() * 0.5;
            break;
        } else if touches.just_released(finger.id()) {
            just_released = true;
            break;
        }
    }

    if buttons.just_pressed(MouseButton::Left) {
        just_pressed = true;
        let window = q_windows.single();
        if let Some(position) = window.cursor_position() {
            left = position.x < window.width() * 0.5;
        }
    } else if buttons.just_released(MouseButton::Left) {
        just_released = true;
    }

    if !just_pressed && !just_released {
        return;
    }

    let (player_entity, mut player, player_transform) = player_q.single_mut();

    if just_released {
        player.tongue_out = false;
    } else if just_pressed {
        player.tongue_out = true;
    }

    if player.tongue_out {
        let start_position = player_transform.translation();
        let mut distance_squared = f32::MAX;
        let mut distance = Vec3::ZERO;
        let mut piller_entity_option = None;
        let mut piller_offset = Vec2::ZERO;

        for (piller_entity, piller_transform, cuboid) in &pillers_q {
            let piller_position = piller_transform.translation();
            if left {
                if start_position.x < piller_position.x {
                    continue;
                }
            } else {
                if start_position.x > piller_position.x {
                    continue;
                }
            }
            if let Some(cuboid_bottom_position) = cuboid
                .shape()
                .as_cuboid()
                .map(|c| Vec3::new(0.0, -c.half_extents.y, 0.0))
            {
                let position = piller_position + cuboid_bottom_position;
                let check_distance: Vec3 = position - start_position;
                let check_distance_squared = check_distance.length_squared();
                if check_distance_squared.is_nan() {
                    continue;
                }
                if check_distance_squared < distance_squared {
                    distance_squared = check_distance_squared;
                    distance = check_distance;
                    piller_entity_option.replace(piller_entity);
                    piller_offset.x = cuboid_bottom_position.x;
                    piller_offset.y = cuboid_bottom_position.y;
                }
            }
        }
        if let Some(piller_entity) = piller_entity_option {
            let distance_length = distance_squared.sqrt();
            let distance_normalized = distance / distance_length;

            let mut parts = distance_length.floor();
            let size: f32 = distance_length / parts;
            let mut trimed = 0.0;
            if parts > 2.0 {
                trimed = parts.sqrt() * 0.5;
                parts -= trimed;
                parts = parts.floor();
            }
            trimed = trimed / parts;

            let mut last_entity = player_entity;

            for i in 0..(parts as usize) {
                let offset = (i as f32 + 1.0 + trimed) * size;
                let position = start_position + distance_normalized * offset;
                let rotation = Quat::from_rotation_z(
                    distance_normalized.y.atan2(distance_normalized.x) - PI * 0.5,
                );
                let collider = Collider::rectangle(0.1, size);
                let entity = commands
                    .spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::RED,
                                custom_size: Some(Vec2::new(0.1, size)),
                                ..default()
                            },
                            transform: Transform::from_translation(position)
                                .with_rotation(rotation),
                            ..default()
                        },
                        TonguePart,
                        RigidBody::Dynamic,
                        CollisionLayers::new(
                            [CollisionLayer::Rope],
                            [CollisionLayer::Ground, CollisionLayer::Piller],
                        ),
                        collider.mass_properties(0.001),
                        collider,
                    ))
                    .id();

                commands.spawn((
                    RevoluteJoint::new(last_entity, entity)
                        .with_compliance(if i == 0 { 0.01 } else { 0.001 })
                        .with_local_anchor_1(Vec2::new(0.0, if i == 0 { 0.0 } else { size * 0.5 }))
                        .with_local_anchor_2(Vec2::new(0.0, size * -0.5)),
                    TongueJoint,
                ));

                last_entity = entity;
            }
            commands.spawn((
                RevoluteJoint::new(last_entity, piller_entity)
                    .with_compliance(0.01)
                    .with_local_anchor_1(Vec2::new(0.0, size * 0.5))
                    .with_local_anchor_2(piller_offset),
                TongueJoint,
            ));
        } else {
            player.tongue_out = false;
        }
    } else {
        for entity in &tongue_q {
            commands.entity(entity).despawn_recursive();
        }
        for entity in &tongue_joints_q {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn sync_player_camera_offset(
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut player_camera_offset: ResMut<PlayerCameraOffset>,
) {
    let window = q_window.single();
    player_camera_offset.x = window.width() * INV_WORLD_SIZE * 0.25;
}

pub fn camera_follow_player(
    q_player: Query<&GlobalTransform, With<Player>>,
    mut q_camera: Query<(&mut Transform, &GlobalTransform), With<Camera>>,
    player_camera_offset: Res<PlayerCameraOffset>,
    time: Res<Time>,
) {
    let player_transform = q_player.single();
    let end_x = player_transform.translation().x + player_camera_offset.x;
    let (mut camera_transform, camera_global_transform) = q_camera.single_mut();
    let start_x = camera_global_transform.translation().x;
    let delta_ms = time.delta_seconds();
    let diff = end_x - start_x;
    if diff == 0.0 {
        return;
    }
    // let change = diff * 1.0 / diff.sqrt() * delta_ms;
    let change = diff * delta_ms;

    camera_transform.translation.x += change;
}
