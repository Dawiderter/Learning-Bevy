#![allow(clippy::type_complexity)]

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

const PADDING: f32 = 50.;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// trait GameUi {
//     fn left_menu(&self) -> Self;
//     fn button
// }

#[derive(Component)]
struct BuildingsButton;

#[derive(Component)]
struct BackButton;

#[derive(Component)]
struct MainChooseMenu;
#[derive(Component)]
struct BuildingsChooseMenu;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        size: Size::new(Val::Px(200.0), Val::Px(100.0)),
        // center button
        margin: UiRect::all(Val::Px(50.)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text = TextStyle {
        font: asset_server.load("fonts/FiraCode-Regular.ttf"),
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(30.), Val::Percent(100.)),
                    ..default()
                },
                background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                ..default()
            })
            .with_children(|left_menu| {
                left_menu
                    .spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_content: AlignContent::SpaceAround,
                                flex_wrap: FlexWrap::Wrap,
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            ..default()
                        },
                        MainChooseMenu,
                    ))
                    .with_children(|button_choose_menu| {
                        button_choose_menu
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                BuildingsButton,
                            ))
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section(
                                    "Budynki",
                                    button_text.clone(),
                                ));
                            });

                        button_choose_menu
                            .spawn(ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section(
                                    "Jednostki",
                                    button_text.clone(),
                                ));
                            });

                        button_choose_menu
                            .spawn(ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .with_children(|button| {
                                button
                                    .spawn(TextBundle::from_section("Zasoby", button_text.clone()));
                            });
                    });
                left_menu
                    .spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_content: AlignContent::SpaceAround,
                                flex_wrap: FlexWrap::Wrap,
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            visibility: Visibility::INVISIBLE,
                            ..default()
                        },
                        BuildingsChooseMenu,
                    ))
                    .with_children(|button_buildings_menu| {
                        button_buildings_menu
                            .spawn(ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section(
                                    "Koszary",
                                    button_text.clone(),
                                ));
                            });

                        button_buildings_menu
                            .spawn(ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .with_children(|button| {
                                button
                                    .spawn(TextBundle::from_section("Ratusz", button_text.clone()));
                            });

                        button_buildings_menu
                            .spawn(ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section(
                                    "Kopalnia",
                                    button_text.clone(),
                                ));
                            });

                        button_buildings_menu
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                BackButton,
                            ))
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section("Wróć", button_text.clone()));
                            });
                    });
            });
        });
}

fn building_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<BuildingsButton>)>,
    mut main_menu_query: Query<
        &mut Visibility,
        (With<MainChooseMenu>, Without<BuildingsChooseMenu>),
    >,
    mut buildings_menu_query: Query<
        &mut Visibility,
        (With<BuildingsChooseMenu>, Without<MainChooseMenu>),
    >,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            for mut main_menu in main_menu_query.iter_mut() {
                *main_menu = Visibility::INVISIBLE;
            }
            for mut buildings_menu in buildings_menu_query.iter_mut() {
                *buildings_menu = Visibility::VISIBLE;
            }
        }
    }
}

fn back_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut main_menu_query: Query<
        &mut Visibility,
        (With<MainChooseMenu>, Without<BuildingsChooseMenu>),
    >,
    mut buildings_menu_query: Query<
        &mut Visibility,
        (With<BuildingsChooseMenu>, Without<MainChooseMenu>),
    >,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            for mut main_menu in main_menu_query.iter_mut() {
                *main_menu = Visibility::VISIBLE;
            }
            for mut buildings_menu in buildings_menu_query.iter_mut() {
                *buildings_menu = Visibility::INVISIBLE;
            }
        }
    }
}

fn button_color_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size().y)
                .sum();
            let panel_height = uinode.size().y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(mouse_scroll)
            .add_system(building_button_system)
            .add_system(button_color_system)
            .add_system(back_button_system);
    }
}
