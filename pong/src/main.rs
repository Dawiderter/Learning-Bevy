use ball::{BallCollider, BallPlugin};
use bevy::prelude::*;
use bevy_kira_audio::prelude::Audio;
use bevy_kira_audio::prelude::*;
use player::{PlayerBundle, PlayerCollider, PlayerPlugin};

mod ball;
mod player;

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
    audio_handle : Handle<AudioSource>
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ScoreText,
        TextBundle::from_section(
            "0 - 0",
            TextStyle {
                font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER),
    ));

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

fn ball_player_collider_system(
    mut ball_query: Query<(&Transform, &mut Velocity, &BallCollider)>,
    player_query: Query<(&Transform, &PlayerCollider)>,
    audio: Res<Audio>,
    ball_sound: Res<BallSound>
) {
    for (b_tr, mut b_vel, b_col) in ball_query.iter_mut() {
        for (p_tr, p_col) in player_query.iter() {
            let pl_bot_left =
                p_tr.transform_point(Vec3::new(-p_col.width / 2., -p_col.height / 2., 0.0));
            let pl_top_right =
                p_tr.transform_point(Vec3::new(p_col.width / 2., p_col.height / 2., 0.0));

            let b_bot_left = b_tr.transform_point(Vec3::new(-b_col.radius, -b_col.radius, 0.0));
            let b_top_right = b_tr.transform_point(Vec3::new(b_col.radius, b_col.radius, 0.0));

            if !(pl_top_right.x < b_bot_left.x
                || b_top_right.x < pl_bot_left.x
                || pl_top_right.y < b_bot_left.y
                || b_top_right.y < pl_bot_left.y)
            {
                *b_vel = Velocity {
                    direction: Vec2::new(b_vel.direction.x * -1., b_vel.direction.y),
                    ..*b_vel
                };
                audio.play(ball_sound.audio_handle.clone());
            }
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.direction.extend(0.0) * velocity.speed;
    }
}

fn setup_players(mut commands: Commands, windows: Res<Windows>,) {
    let window = windows.get_primary().unwrap();

    let first_player_x = -window.width() / 2. + PLAYER_FROM_EDGE_MARGIN;
    let second_player_x = window.width() / 2. - PLAYER_FROM_EDGE_MARGIN;

    let starting_y = 0.;

    commands.spawn((
        Player1,
        PlayerBundle::default()
            .with_start_pos(Vec2::new(first_player_x, starting_y))
            .with_keys(KeyCode::W, KeyCode::S),
    ));

    commands.spawn((
        Player2,
        PlayerBundle::default()
            .with_start_pos(Vec2::new(second_player_x, starting_y))
            .with_keys(KeyCode::Up, KeyCode::Down),
    ));
}

fn setup_assets(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(BallSound {
        audio_handle: server.load("sounds/pong.mp3"),
    })
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1200.0,
                height: 600.0,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_players)
        .add_startup_system(setup_assets)
        .add_plugin(BallPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AudioPlugin)
        .add_system(apply_velocity.before("Collider"))
        .add_system(ball_player_collider_system.label("Collider"))
        .add_startup_system(setup_ui)
        .add_system(update_score_ui)
        .run();
}
