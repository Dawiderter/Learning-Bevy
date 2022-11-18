use ball::BallPlugin;
//Dla wersji FixedTimestep
//use bevy::{ecs::schedule::ShouldRun, time::FixedTimestep};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::egui::{Frame, Pos2};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_kira_audio::prelude::*;
use collisions::{CollisionPhase, CollisionPlugin};
use player::{AiInputComp, PlayerBundle, PlayerInputComp, PlayerPlugin};

mod ball;
mod collisions;
mod player;

const PLAYER_FROM_EDGE_MARGIN: f32 = 40.;

#[derive(Component)]
struct Velocity {
    direction: Vec2,
    speed: f32,
}

#[derive(Component)]
struct Score {
    player1_score: i32,
    player2_score: i32,
}

#[derive(Component)]
struct ScoreText;

#[derive(Resource)]
struct BallSound {
    audio_handle: Handle<AudioSource>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum GameState {
    InGame,
    Paused,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ScoreText,
                TextBundle::from_section(
                    "0 - 0",
                    TextStyle {
                        font: asset_server.load("fonts/FiraCode-Regular.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::TOP_CENTER)
                .with_style(Style {
                    size: Size::new(Val::Undefined, Val::Px(25.)),
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..default()
                    },
                    ..default()
                }),
            ));
        });

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

// Dla wersji wykonywanej stałą ilość razy w ciągu sekundy
// fn apply_velocity_fixed(mut query: Query<(&mut Transform, &Velocity)>) {
//     for (mut transform, velocity) in query.iter_mut() {
//         transform.translation +=
//             velocity.direction.extend(0.0) * velocity.speed * TIME_STEP;
//     }
// }

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation +=
            velocity.direction.extend(0.0) * velocity.speed * time.delta_seconds();
    }
}

fn setup_players(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    let first_player_x = -window.width() / 2. + PLAYER_FROM_EDGE_MARGIN;
    let second_player_x = window.width() / 2. - PLAYER_FROM_EDGE_MARGIN;

    let starting_y = 0.;

    commands.spawn((
        AiInputComp,
        PlayerBundle::default().with_start_pos(Vec2::new(first_player_x, starting_y)),
    ));

    commands.spawn((
        PlayerInputComp,
        PlayerBundle::default()
            .with_start_pos(Vec2::new(first_player_x / 2., starting_y))
            .with_keys(KeyCode::E, KeyCode::D),
    ));

    commands.spawn((
        PlayerInputComp,
        PlayerBundle::default()
            .with_start_pos(Vec2::new(second_player_x, starting_y))
            .with_keys(KeyCode::I, KeyCode::K),
    ));

    commands.spawn((
        PlayerInputComp,
        PlayerBundle::default()
            .with_start_pos(Vec2::new(second_player_x / 2., starting_y))
            .with_keys(KeyCode::U, KeyCode::J),
    ));
}

fn pause_system(mut app_state: ResMut<State<GameState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::P) {
        match app_state.current() {
            GameState::Paused => {
                app_state.pop().unwrap();
            }
            GameState::InGame => {
                app_state.push(GameState::Paused).unwrap();
            }
        };
    }
}

fn draw_pause_menu(
    mut egui_context: ResMut<EguiContext>,
    app_state: Res<State<GameState>>,
    windows: Res<Windows>,
) {
    if app_state.current() != &GameState::Paused {
        return;
    }
    use egui::*;
    let ctx = egui_context.ctx_mut();
    let text_color = ctx.style().visuals.text_color();

    let height = 28.0;
    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            // Paint the frame:
            painter.rect(
                rect.shrink(1.0),
                10.0,
                ctx.style().visuals.window_fill(),
                Stroke::new(1.0, text_color),
            );

            // Paint the title:
            painter.text(
                rect.center_top() + vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                "Title",
                FontId::proportional(height * 0.8),
                text_color,
            );

            // Paint the line under the title:
            painter.line_segment(
                [
                    rect.left_top() + vec2(2.0, height),
                    rect.right_top() + vec2(-2.0, height),
                ],
                Stroke::new(1.0, text_color),
            );

            // Add the close button:
            let close_response = ui.put(
                Rect::from_min_size(rect.left_top(), Vec2::splat(height)),
                Button::new(RichText::new("❌").size(height - 4.0)).frame(false),
            );
            if close_response.clicked() {}

            // Interact with the title bar (drag to move window):
            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + height;
                rect
            };
            let title_bar_response =
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {}

            // Add the contents:
        });
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
        .add_state(GameState::InGame)
        .add_plugin(EguiPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(CollisionPlugin)
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(apply_velocity.before(CollisionPhase)),
        )
        // Dla wykonywania systemów stałą ilość razy w ciągu sekundy
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(TIME_STEP as f64).pipe(
        //             |In(input): In<ShouldRun>, state: Res<State<GameState>>| {
        //                 if state.current() == &GameState::InGame {
        //                     input
        //                 } else {
        //                     ShouldRun::No
        //                 }
        //             },
        //         ))
        //         .with_system(apply_velocity.before(CollisionPhase)),
        // )
        .add_startup_system(setup_ui)
        .add_system(pause_system)
        .add_system(update_score_ui)
        .add_system(draw_pause_menu)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
