use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{collisions::PlayerCollider, GameState};

const PLAYERS_SPEED: f32 = 5.0;
const PLAYER_WIDTH: f32 = 10.0;
const PLAYER_HEIGHT: f32 = 120.0;
pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerInput {
    Up,
    Down,
}

#[derive(Component)]
pub struct PlayerInputComp;

#[derive(Component)]
pub struct AiInputComp;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    pub sprite_bundle: SpriteBundle,
    pub player_collider: PlayerCollider,
    pub input_manager_bundle: InputManagerBundle<PlayerInput>,
}

impl PlayerBundle {
    pub fn with_start_pos(mut self, pos: Vec2) -> Self {
        self.sprite_bundle.transform.translation = pos.extend(0.);
        self
    }
    pub fn with_keys(mut self, up_key: KeyCode, down_key: KeyCode) -> Self {
        self.input_manager_bundle
            .input_map
            .insert(up_key, PlayerInput::Up)
            .insert(down_key, PlayerInput::Down);
        self
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.8, 1.0),
                    custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            },
            player_collider: PlayerCollider {
                width: PLAYER_WIDTH,
                height: PLAYER_HEIGHT,
            },
            input_manager_bundle: InputManagerBundle::<PlayerInput> {
                action_state: ActionState::default(),
                input_map: InputMap::default(),
            },
        }
    }
}

fn player_movement_system(mut query: Query<(&mut Transform, &mut ActionState<PlayerInput>)>) {
    for (mut transform, action_state) in query.iter_mut() {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        if action_state.pressed(PlayerInput::Up) {
            direction = Vec3::new(0.0, 1.0, 0.0);
        } else if action_state.pressed(PlayerInput::Down) {
            direction = Vec3::new(0.0, -1.0, 0.0);
        }

        transform.translation += direction * PLAYERS_SPEED;
    }
}

fn player_ai_system(
    mut query: Query<(&Transform, &mut ActionState<PlayerInput>), With<AiInputComp>>,
) {
    for (transform, mut action_state) in query.iter_mut() {
        action_state.press(PlayerInput::Up);
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerInput>::default())
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(player_movement_system)
                    .with_system(player_ai_system),
            );
    }
}
