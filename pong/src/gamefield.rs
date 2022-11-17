use bevy::prelude::*;

pub const DEFAULT_WIDTH_TO_HEIGHT: f32 = 16./9.;
pub const DEFAULT_WIDTH: f32 = 1600.0;
pub const DEFAULT_OFFSET: Offset = Offset{up: 0.0, down: 0.0, left: 0.0, right: 0.0};

#[derive(Component)]
pub struct GameFieldSettigns {
    pub offset: Offset,
    pub width_to_height: f32,
}

pub struct Offset {
    up: f32,
    down: f32,
    left: f32,
    right: f32,
}

#[derive(Component)]
pub struct  GameField;

#[derive(Bundle)]
pub struct GameFieldBundle {
    pub game_field: GameField,
    pub sprite_bundle: SpriteBundle,
    pub settings: GameFieldSettigns,
}

pub fn update_gamefield (
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, & GameFieldSettigns), With<GameField>>) {
        let window = windows.get_primary().unwrap();

        for (mut transform, settings) in query.iter_mut() {
            transform.translation = Vec3 {
                x: window.width()/2.0 * (settings.offset.left - settings.offset.right), 
                y:  window.height()/2.0 * (settings.offset.down - settings.offset.up), 
                z:0.0};

            let scale_from_width = (window.width() * (1.0 - settings.offset.left - settings.offset.right)) / DEFAULT_WIDTH;
            let scale_from_height = (window.height() * (1.0 - settings.offset.down - settings.offset.up)) / DEFAULT_WIDTH*DEFAULT_WIDTH_TO_HEIGHT;

            transform.scale = Vec3::splat(scale_from_height.min(scale_from_width));
        }
    }

impl Default for GameFieldBundle {
    fn default() -> Self {
        Self {game_field: GameField,sprite_bundle: SpriteBundle{sprite: Sprite {
            color: Color::rgb(0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(DEFAULT_WIDTH, DEFAULT_WIDTH/DEFAULT_WIDTH_TO_HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()},
        settings: GameFieldSettigns { offset: DEFAULT_OFFSET, width_to_height: DEFAULT_WIDTH_TO_HEIGHT }}
    }
}