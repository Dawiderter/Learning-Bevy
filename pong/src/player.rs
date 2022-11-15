use bevy::prelude::*;

const PLAYER_FROM_EDGE_MARGIN: f32 = 40.;
pub const PLAYERS_SPEED: f32 = 5.0;
const PLAYER_WIDTH: f32 = 10.0;
const PLAYER_HEIGHT: f32 = 120.0;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player1;

#[derive(Component)]
pub struct Player2;

#[derive(Component)]
pub struct PlayerCollider {
    pub width: f32,
    pub height: f32,
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

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_players);
    }
}
