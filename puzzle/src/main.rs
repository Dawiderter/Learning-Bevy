mod animation;

use std::{collections::HashMap, hash::Hash};

use animation::{
    advance_animation_system, apply_animation_system, AnimationData, AnimationStateMachine,
};
use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Resource, Default)]
struct MainAtlas {
    handle: Handle<TextureAtlas>,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum PlayerDir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum PlayerState {
    Walk,
    Idle,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_assets(
    server: Res<AssetServer>,
    mut main_atlas: ResMut<MainAtlas>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = server.load("atlases/sokoban.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(128., 128.), 13, 8, None, None);

    let atlas_handle = atlases.add(texture_atlas);

    main_atlas.handle = atlas_handle;
}

fn setup_player(mut commands: Commands, main_atlas: Res<MainAtlas>) {
    commands.spawn((
        Player,
        SpriteSheetBundle {
            texture_atlas: main_atlas.handle.clone(),
            ..default()
        },
        AnimationStateMachine::from_data(
            [
                (
                    (PlayerDir::Down, PlayerState::Walk),
                    AnimationData::from_frames(53, 2)
                        .with_dur_sec(0.2)
                        .repeating(),
                ),
                (
                    (PlayerDir::Up, PlayerState::Walk),
                    AnimationData::from_frames(56, 2)
                        .with_dur_sec(0.2)
                        .repeating(),
                ),
                (
                    (PlayerDir::Right, PlayerState::Walk),
                    AnimationData::from_frames(79, 2)
                        .with_dur_sec(0.2)
                        .repeating(),
                ),
                (
                    (PlayerDir::Left, PlayerState::Walk),
                    AnimationData::from_frames(82, 2)
                        .with_dur_sec(0.2)
                        .repeating(),
                ),
                (
                    (PlayerDir::Down, PlayerState::Idle),
                    AnimationData::from_frames(52, 1).repeating(),
                ),
                (
                    (PlayerDir::Up, PlayerState::Idle),
                    AnimationData::from_frames(55, 1).repeating(),
                ),
                (
                    (PlayerDir::Right, PlayerState::Idle),
                    AnimationData::from_frames(78, 1).repeating(),
                ),
                (
                    (PlayerDir::Left, PlayerState::Idle),
                    AnimationData::from_frames(81, 1),
                ),
            ],
            (PlayerDir::Down, PlayerState::Idle),
        ),
    ));
}

fn player(
    mut query: Query<(
        &mut Transform,
        &mut AnimationStateMachine<(PlayerDir, PlayerState)>,
    )>,
    input: Res<Input<KeyCode>>,
) {
    for (mut transform, mut anim) in query.iter_mut() {
        let mut dir = (0, 0);

        if input.pressed(KeyCode::D) {
            dir.0 += 1;
        }
        if input.pressed(KeyCode::W) {
            dir.1 += 1;
        }
        if input.pressed(KeyCode::S) {
            dir.1 -= 1;
        }
        if input.pressed(KeyCode::A) {
            dir.0 -= 1;
        }

        let mut state = anim.current_state;

        match dir {
            (_, 1) => {
                state = (PlayerDir::Up, PlayerState::Walk);
            }
            (_, -1) => {
                state = (PlayerDir::Down, PlayerState::Walk);
            }
            (1, 0) => {
                state = (PlayerDir::Right, PlayerState::Walk);
            }
            (-1, 0) => {
                state = (PlayerDir::Left, PlayerState::Walk);
            }
            _ => {
                state = (state.0, PlayerState::Idle);
            }
        }

        anim.switch_state(state);

        transform.translation += Vec3::new(dir.0 as f32 * 5., dir.1 as f32 * 5., 0.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<MainAtlas>()
        .add_startup_system(setup_camera)
        .add_startup_system(setup_player.label("player"))
        .add_startup_system(setup_assets.before("player"))
        .add_system(player)
        .add_system(advance_animation_system::<AnimationStateMachine<(PlayerDir, PlayerState)>>)
        .add_system(apply_animation_system::<AnimationStateMachine<(PlayerDir, PlayerState)>>)
        .run();
}
