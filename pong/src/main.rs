use bevy::prelude::*;

const PLAYER_FROM_EDGE_MARGIN: f32 = 40.;
const PLAYERS_SPEED: f32 = 5.0;
const PLAYER_WIDTH: f32 = 50.0;
const PLAYER_HEIGHT: f32 = 120.0;
const BALL_DIAMETER: f32 = 40.0;
const BALL_SPEED: f32 = 5.0;

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity {
    direction: Vec2,
    speed: f32,
}

#[derive(Component)]
struct PlayerCollider {
    width: f32,
    height: f32,
}

#[derive(Component)]
struct BallCollider {
    radius: f32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_players(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    let first_player_x = -window.width() / 2. + PLAYER_FROM_EDGE_MARGIN;
    let second_player_x = window.width() / 2. - PLAYER_FROM_EDGE_MARGIN;

    let starting_y = 0.;

    commands.spawn((
        Player1,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 1.0),
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(first_player_x, starting_y, 0.0)),
            ..default()
        },
        PlayerCollider {
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
        },
    ));

    commands.spawn((
        Player2,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 1.0),
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(second_player_x, starting_y, 0.0)),
            ..default()
        },
        PlayerCollider {
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
        },
    ));
}

fn setup_ball(mut commands: Commands) {
    commands.spawn((
        Ball,
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

fn first_player_system(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player1>>) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        if keys.pressed(KeyCode::W) {
            direction = Vec3::new(0.0, 1.0, 0.0)
        } else if keys.pressed(KeyCode::S) {
            direction = Vec3::new(0.0, -1.0, 0.0)
        }

        transform.translation += direction * PLAYERS_SPEED;
    }
}

fn second_player_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player2>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        if keys.pressed(KeyCode::Up) {
            direction = Vec3::new(0.0, 1.0, 0.0)
        } else if keys.pressed(KeyCode::Down) {
            direction = Vec3::new(0.0, -1.0, 0.0)
        }

        transform.translation += direction * PLAYERS_SPEED;
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

fn ball_reset_system(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Ball>>) {
    if keys.just_pressed(KeyCode::R) {
        for mut transform in query.iter_mut() {
            transform.translation = Vec3::new(0., 0., 0.);
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

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.direction.extend(0.0) * velocity.speed;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.0,
                height: 600.0,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_players)
        .add_startup_system(setup_ball)
        .add_system(first_player_system)
        .add_system(second_player_system)
        .add_system(apply_velocity.before("Collider"))
        .add_system(ball_wall_collider_system.label("Collider"))
        .add_system(ball_player_collider_system.label("Collider"))
        .add_system(ball_reset_system)
        .run();
}
