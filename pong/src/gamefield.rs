use bevy::prelude::*;

const FIELD_WIDTH_TO_HEIGHT: f32 = 16./9.;

#[derive(Component)]
pub struct GameField {
    pub sprite_bundle: SpriteBundle,
}

// fn update_gamefield (
//     windows: Res<Windows>,
//     mut query: Query<(&Transform, &mut Velocity, &BallCollider)>) {

//     }

impl Default for GameField {
    fn default() -> Self {
        Self {sprite_bundle: SpriteBundle{sprite: Sprite {
            color: Color::rgb(0., 0., 1.0),
            custom_size: Some(Vec2::new(1600.0, 900.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()}}
    }
}