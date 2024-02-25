use bevy::prelude::*;

use crate::app::{despawn_component_system, despawn_resource_system, GameState};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(
                Update,
                (countdown, spinner).run_if(in_state(GameState::Splash)),
            )
            .add_systems(
                OnExit(GameState::Splash),
                (
                    despawn_component_system::<OnSplashScreen>,
                    despawn_resource_system::<SplashTimer>,
                ),
            );
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Component)]
struct SlashSpinner;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("spinner.png");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(64.0),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                },
                SlashSpinner,
            ));
        });
    commands.insert_resource(SplashTimer(Timer::from_seconds(0.0, TimerMode::Once)));
}

fn spinner(time: Res<Time>, mut spinner_q: Query<&mut Transform, With<SlashSpinner>>) {
    for mut transform in spinner_q.iter_mut() {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds() * 4.0);
    }
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Game);
    }
}
