use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const PLAYER_FROM_EDGE_MARGIN: f32 = 40.;
const PLAYERS_SPEED: f32 = 5.0;
const PLAYER_WIDTH: f32 = 10.0;
const PLAYER_HEIGHT: f32 = 120.0;
const BALL_DIAMETER: f32 = 10.0;
const BALL_SPEED: f32 = 5.0;

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

#[derive(Component)]
struct Ball {
    active: bool
}

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

#[derive(Component)]
struct Score {
    player1_score: i32,
    player2_score: i32,
}

#[derive(Component)]
struct ScoreText;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Player1State {
    Up,
    Down,
}
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Player2State {
    Up,
    Down,
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
        ).with_text_alignment(TextAlignment::TOP_CENTER),
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

fn first_player_input(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<Player1State> {
        action_state: ActionState::default(),
        input_map: InputMap::new([
            (KeyCode::W, Player1State::Up),
            (KeyCode::S, Player1State::Down),
        ]),
    });
}

fn second_player_input(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<Player2State> {
        action_state: ActionState::default(),
        input_map: InputMap::new([
            (KeyCode::Up, Player2State::Up),
            (KeyCode::Down, Player2State::Down),
        ]),
    });
}

fn first_player_input_system(
    action_query: Query<&ActionState<Player1State>>,
    mut query: Query<&mut Transform, With<Player1>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        let action_state = action_query.single();

        if action_state.pressed(Player1State::Up) {
            direction = Vec3::new(0.0, 1.0, 0.0);
        } else if action_state.pressed(Player1State::Down) {
            direction = Vec3::new(0.0, -1.0, 0.0);
        }

        transform.translation += direction * PLAYERS_SPEED;
    }
}

fn second_player_input_system(
    action_query: Query<&ActionState<Player2State>>,
    mut query: Query<&mut Transform, With<Player2>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        let action_state = action_query.single();

        if action_state.pressed(Player2State::Up) {
            direction = Vec3::new(0.0, 1.0, 0.0);
        } else if action_state.pressed(Player2State::Down) {
            direction = Vec3::new(0.0, -1.0, 0.0);
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

fn ball_reset_system(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &mut Ball)>) {
    if keys.just_pressed(KeyCode::R) {
        for (mut transform, mut ball) in query.iter_mut() {
            transform.translation = Vec3::new(0., 0., 0.);
            ball.active = true;
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
                width: 1200.0,
                height: 600.0,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_players)
        .add_startup_system(setup_ball)
        .add_startup_system(first_player_input)
        .add_startup_system(second_player_input)
        .add_plugin(InputManagerPlugin::<Player1State>::default())
        .add_plugin(InputManagerPlugin::<Player2State>::default())
        .add_system(first_player_input_system)
        .add_system(second_player_input_system)
        .add_system(apply_velocity.before("Collider"))
        .add_system(ball_wall_collider_system.label("Collider"))
        .add_system(ball_player_collider_system.label("Collider"))
        .add_system(ball_reset_system)
        .add_startup_system(setup_ui)
        .add_system(check_for_ball_score)
        .add_system(update_score_ui)
        .run();
}
