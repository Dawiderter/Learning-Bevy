use crate::player::{Player1, Player2, PLAYERS_SPEED};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct Player1InputPlugin;
pub struct Player2InputPlugin;

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

impl Plugin for Player1InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(first_player_input)
            .add_plugin(InputManagerPlugin::<Player1State>::default())
            .add_system(first_player_input_system);
    }
}
impl Plugin for Player2InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(second_player_input)
            .add_plugin(InputManagerPlugin::<Player2State>::default())
            .add_system(second_player_input_system);
    }
}
