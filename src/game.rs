use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;
use rand::prelude::*;

use crate::{
    app::{GameState, WORLD_HEIGHT_UNITS},
    player::{Player, PlayerPlugin},
};

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Rope,
    Player,
    Piller,
    Ground,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_systems(OnEnter(GameState::Game), setup_game);
    }
}

#[derive(Component)]
pub struct Piller;

#[derive(Component)]
pub struct Ground;

fn setup_game(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    let scene_middle_left = camera
        .viewport_to_world_2d(camera_transform, Vec2::new(0.0, window.height() * 0.5))
        .unwrap_or_default();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            transform: Transform::from_xyz(scene_middle_left.x, WORLD_HEIGHT_UNITS * -0.25, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(1.0, 1.0),
        LinearVelocity(Vec2::new(12.0, 12.0)),
        CollisionLayers::new(
            [CollisionLayer::Player],
            [CollisionLayer::Ground, CollisionLayer::Piller],
        ),
        Player { tongue_out: false },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(10000.0, 1.0)),
                ..default()
            },
            transform: Transform::from_xyz(5000.0, WORLD_HEIGHT_UNITS * -0.5, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(10000.0, 1.0),
        CollisionLayers::new([CollisionLayer::Ground], [CollisionLayer::Player]),
        Ground,
    ));

    let mut rng = SmallRng::from_entropy();
    let mut last_x = 0.0;
    for _ in 0..1000 {
        let height = rng.gen_range(WORLD_HEIGHT_UNITS * 0.1..WORLD_HEIGHT_UNITS * 0.75);

        let x = rng.gen_range(last_x..(last_x + WORLD_HEIGHT_UNITS));
        let y = (WORLD_HEIGHT_UNITS - height) * 0.5;

        last_x = x;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(1.0, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            RigidBody::Static,
            Collider::rectangle(1.0, height),
            CollisionLayers::new([CollisionLayer::Piller], [CollisionLayer::Player]),
            Piller,
        ));
    }
}
