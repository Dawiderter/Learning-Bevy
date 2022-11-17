use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{BallSound, GameState, Velocity};

#[derive(Component)]
pub struct BallCollider {
    pub radius: f32,
}

#[derive(Component)]
pub struct PlayerCollider {
    pub width: f32,
    pub height: f32,
}

pub struct CollisionPlugin;

#[derive(SystemLabel)]
pub struct CollisionPhase;

fn ball_player_collider_system(
    mut ball_query: Query<(&Transform, &mut Velocity, &BallCollider)>,
    player_query: Query<(&Transform, &PlayerCollider)>,
    audio: Res<Audio>,
    ball_sound: Res<BallSound>,
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
                    speed: b_vel.speed * 1.1
                };
                audio.play(ball_sound.audio_handle.clone());
            }
        }
    }
}

fn ball_wall_collider_system(
    windows: Res<Windows>,
    mut query: Query<(&Transform, &mut Velocity, &BallCollider)>,
) {
    let window = windows.get_primary().unwrap();

    for (b_tr, mut b_vel, b_col) in query.iter_mut() {
        let b_bot_left = b_tr.transform_point(Vec3::new(-b_col.radius, -b_col.radius, 0.0));
        let b_top_right = b_tr.transform_point(Vec3::new(b_col.radius, b_col.radius, 0.0));

        if b_top_right.y > window.height() / 2. || b_bot_left.y < -window.height() / 2. {
            *b_vel = Velocity {
                direction: Vec2::new(b_vel.direction.x, b_vel.direction.y * -1.),
                ..*b_vel
            }
        }
    }
}

fn setup_assets(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(BallSound {
        audio_handle: server.load("sounds/pong.mp3"),
    })
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(ball_wall_collider_system.label(CollisionPhase))
                .with_system(ball_player_collider_system.label(CollisionPhase)),
        )
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_assets));
    }
}
