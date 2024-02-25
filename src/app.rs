use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_xpbd_2d::{math::Vector, prelude::*};

use crate::{game::GamePlugin, splash::SplashPlugin};

pub const WORLD_SIZE: f32 = 32.0;
pub const INV_WORLD_SIZE: f32 = 1.0 / WORLD_SIZE;
pub const WORLD_HEIGHT_UNITS: f32 = 32.0;
pub const WORLD_HEIGHT: f32 = WORLD_SIZE * WORLD_HEIGHT_UNITS;

pub const GRAVITY_ON_EARTH: f32 = 9.801;
pub const GRAVITY_SCALE: f32 = 1.0;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Game,
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flicky Frog".to_string(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
        ))
        .insert_resource(SubstepCount(10))
        .insert_resource(Gravity(Vector::NEG_Y * GRAVITY_ON_EARTH * GRAVITY_SCALE))
        .insert_resource(Msaa::Off)
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_plugins((SplashPlugin, GamePlugin));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.0,
            near: -1000.0,
            scale: INV_WORLD_SIZE,
            scaling_mode: ScalingMode::FixedVertical(WORLD_HEIGHT),
            ..default()
        },
        ..default()
    });
}

pub fn despawn_component_system<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_resource_system<T: Resource>(mut commands: Commands) {
    commands.remove_resource::<T>();
}
