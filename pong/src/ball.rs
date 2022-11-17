use bevy::prelude::*;

use crate::{ GameState, Score, Velocity, gamefield::DEFAULT_WIDTH};

pub const BALL_DIAMETER: f32 = 10.0;
pub const BALL_SPEED: f32 = 500.0;

pub struct BallPlugin;

#[derive(Component)]
pub struct Ball {
    pub active: bool,
}

fn check_for_ball_score(
    mut score_query: Query<&mut Score>,
    mut ball_query: Query<(&Transform, &mut Ball)>,
) {

    let mut score = score_query.get_single_mut().unwrap();
    for (ball_trans, mut ball) in ball_query.iter_mut() {
        if ball.active {
            if ball_trans.translation.x > DEFAULT_WIDTH / 2. {
                score.player1_score += 1;
                ball.active = false;
            } else if ball_trans.translation.x < -DEFAULT_WIDTH / 2. {
                score.player2_score += 1;
                ball.active = false;
            }
        }
    }
}

fn ball_reset_system(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &mut Ball, &mut Velocity)>) {
    if keys.just_pressed(KeyCode::R) {
        for (mut transform, mut ball, mut vel) in query.iter_mut() {
            transform.translation = Vec3::new(0., 0., 10.);
            ball.active = true;
            vel.speed = BALL_SPEED;
        }
    }
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(ball_reset_system)
                    .with_system(check_for_ball_score),
            );
    }
}
