use bevy::prelude::*;

use crate::{Score, Velocity};

const BALL_DIAMETER: f32 = 10.0;
const BALL_SPEED: f32 = 5.0;

pub struct BallPlugin;

#[derive(Component)]
pub struct Ball {
    active: bool,
}

#[derive(Component)]
pub struct BallCollider {
    pub radius: f32,
}

fn setup_ball(mut commands: Commands) {
    commands.spawn((
        Ball { active: true },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 1.0),
                custom_size: Some(Vec2::new(BALL_DIAMETER, BALL_DIAMETER)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
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

fn check_for_ball_score(
    mut score_query: Query<&mut Score>,
    mut ball_query: Query<(&Transform, &mut Ball)>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    let mut score = score_query.get_single_mut().unwrap();
    for (ball_trans, mut ball) in ball_query.iter_mut() {
        if ball.active {
            if ball_trans.translation.x > window.width() / 2. {
                score.player1_score += 1;
                ball.active = false;
            } else if ball_trans.translation.x < -window.width() / 2. {
                score.player2_score += 1;
                ball.active = false;
            }
        }
    }
}

fn ball_reset_system(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &mut Ball)>) {
    if keys.just_pressed(KeyCode::R) {
        for (mut transform, mut ball) in query.iter_mut() {
            transform.translation = Vec3::new(0., 0., 0.);
            ball.active = true;
        }
    }
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ball)
            .add_system(ball_wall_collider_system.label("Collider"))
            .add_system(ball_reset_system)
            .add_system(check_for_ball_score);
    }
}
