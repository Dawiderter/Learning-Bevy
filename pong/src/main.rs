use ball::{BallPlugin, Ball, BALL_DIAMETER, BALL_SPEED};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use collisions::{CollisionPlugin, CollisionPhase, BallCollider};
use gamefield::{GameFieldBundle, update_gamefield};
use player::{PlayerBundle, PlayerPlugin};

mod ball;
mod player;
mod collisions;
mod gamefield;

const PLAYER_FROM_EDGE_MARGIN: f32 = 40.;

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

#[derive(Component)]
struct Velocity {
    direction: Vec2,
    speed: f32,
}

#[derive(Component)]
struct Score {
    player1_score: i32,
    player2_score: i32,
}

#[derive(Component)]
struct ScoreText;

#[derive(Resource)]
struct BallSound {
    audio_handle: Handle<AudioSource>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum GameState {
    InGame,
    Paused,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ScoreText,
                TextBundle::from_section(
                    "0 - 0",
                    TextStyle {
                        font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::TOP_CENTER)
                .with_style(Style {
                    size: Size::new(Val::Undefined, Val::Px(25.)),
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..default()
                    },
                    ..default()
                }),
            ));
        });

    commands.spawn(Score {
        player1_score: 0,
        player2_score: 0,
    });
}

fn update_score_ui(mut text_query: Query<&mut Text, With<ScoreText>>, score_query: Query<&Score>) {
    let score = score_query.get_single().unwrap();
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{} - {}", score.player1_score, score.player2_score);
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.direction.extend(0.0) * velocity.speed * time.delta_seconds();
    }
}

fn setup_gamefield(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    let first_player_x = -window.width() / 2. + PLAYER_FROM_EDGE_MARGIN;
    let second_player_x = window.width() / 2. - PLAYER_FROM_EDGE_MARGIN;

    let starting_y = 0.;

    commands.spawn(GameFieldBundle::default()).with_children(|parent| { 
        parent.spawn((
            Player1,
            PlayerBundle::default()
                .with_start_pos(Vec3::new(first_player_x, starting_y, 10.0))
                .with_keys(KeyCode::W, KeyCode::S))); 

        parent.spawn((
            Player1,
            PlayerBundle::default()
                .with_start_pos(Vec3::new(first_player_x/2., starting_y, 10.0))
                .with_keys(KeyCode::E, KeyCode::D),
        ));
    
        parent.spawn((
            Player2,
            PlayerBundle::default()
                .with_start_pos(Vec3::new(second_player_x, starting_y, 10.0))
                .with_keys(KeyCode::I, KeyCode::K),
        ));
    
        parent.spawn((
            Player2,
            PlayerBundle::default()
                .with_start_pos(Vec3::new(second_player_x/2., starting_y, 10.0))
                .with_keys(KeyCode::U, KeyCode::J),
        ));

        parent.spawn((
            Ball { active: true },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.8, 1.0),
                    custom_size: Some(Vec2::new(BALL_DIAMETER, BALL_DIAMETER)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
                ..default()
            },
            BallCollider {
                radius: BALL_DIAMETER / 2.,
            },
            Velocity {
                direction: Vec2::new(1.0, 1.0).normalize(),
                speed: BALL_SPEED,
            },
        ));

        });

}

fn pause_system(mut app_state: ResMut<State<GameState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        match app_state.current() {
            GameState::Paused => {
                app_state.pop().unwrap();
            }
            GameState::InGame => {
                app_state.push(GameState::Paused).unwrap();
            }
        };
    }
} 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1600.0,
                height: 900.0,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_gamefield)
        .add_state(GameState::InGame)
        .add_plugin(BallPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(CollisionPlugin)
        .add_system_set(SystemSet::on_update(GameState::InGame).with_system(apply_velocity.before(CollisionPhase)))
        .add_startup_system(setup_ui)
        .add_system(update_score_ui)
        .add_system(pause_system)
        .add_system(update_gamefield)
        .run();
}
