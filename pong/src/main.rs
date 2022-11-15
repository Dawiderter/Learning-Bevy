use bevy::prelude::*;
use ball::{BallCollider, BallPlugin};
use player::{PlayerCollider, PlayerPlugin};
use player_input::{Player1InputPlugin, Player2InputPlugin};

mod ball;
mod player;
mod player_input;

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
                }
            }
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.direction.extend(0.0) * velocity.speed;
    }
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
        .add_plugin(BallPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(Player1InputPlugin)
        .add_plugin(Player2InputPlugin)
        .add_system(apply_velocity.before("Collider"))
        .add_system(ball_player_collider_system.label("Collider"))
        .add_startup_system(setup_ui)
        .add_system(update_score_ui)
        .run();
}
