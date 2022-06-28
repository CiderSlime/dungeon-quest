use bevy::prelude::*;

use crate::config::*;
use crate::resources::dictionary::Dictionary;
use crate::scenes::SceneState;

const LOADING_TEXT_FONT_SIZE: f32 = 30.0;
const TEXT_FONT_SIZE: f32 = 40.0;

const LOADING_BORDER_WIDTH: f32 = 600.0;
const LOADING_BORDER_HEIGHT: f32 = 60.0;

#[derive(Component)]
struct Loader {
    max_width: f32,
    current_width: f32,
}

struct LoadingSceneData {
    ui_root: Entity,
}

pub struct LoadingScenePlugin;

impl Plugin for LoadingScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::LoadingScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::LoadingScene).with_system(update_loader),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::LoadingScene).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, dictionary: Res<Dictionary>) {
    let ui_root = commands
        .spawn_bundle(root())
        .with_children(|parent| {
            loading_text(parent, &asset_server, &dictionary);
            loader_bundle(parent, &asset_server, &dictionary);
        })
        .id();

    commands.insert_resource(LoadingSceneData { ui_root });
}

fn cleanup(mut commands: Commands, loading_scene_data: Res<LoadingSceneData>) {
    commands
        .entity(loading_scene_data.ui_root)
        .despawn_recursive();
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        color: UiColor(Color::BLACK),
        ..Default::default()
    }
}

fn loader_bundle(
    root: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    dictionary: &Res<Dictionary>,
) {
    root.spawn_bundle(
        // Border
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                size: Size::new(
                    Val::Px(LOADING_BORDER_WIDTH),
                    Val::Px(LOADING_BORDER_HEIGHT),
                ),
                position: Rect {
                    top: Val::Px((WINDOW_HEIGHT / 2.0) - (LOADING_BORDER_HEIGHT / 2.0)),
                    left: Val::Px(
                        (WINDOW_HEIGHT * RESOLUTION) / 2.0 - (LOADING_BORDER_WIDTH / 2.0),
                    ),
                    bottom: Val::Auto,
                    right: Val::Auto,
                },
                ..Default::default()
            },
            color: UiColor(Color::DARK_GRAY),
            ..Default::default()
        },
    )
    .with_children(|parent| {
        // Loader
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    size: Size::new(
                        Val::Px(0.0),
                        Val::Px(LOADING_BORDER_HEIGHT - LOADING_BORDER_HEIGHT * 0.2),
                    ),
                    position: Rect {
                        left: Val::Px(5.0),
                        top: Val::Px(5.0),
                        bottom: Val::Px(5.0),
                        right: Val::Px(5.0),
                    },
                    ..Default::default()
                },
                color: UiColor(Color::rgb(247.0 / 255.0, 104.0 / 255.0, 12.0 / 255.0)),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: asset_server.load(dictionary.get_font()),
                            font_size: TEXT_FONT_SIZE,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            })
            .insert(Loader {
                max_width: LOADING_BORDER_WIDTH - 10.0,
                current_width: 0.0,
            });
    });
}

fn loading_text(
    root: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    dictionary: &Res<Dictionary>,
) {
    root.spawn_bundle(NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(LOADING_BORDER_WIDTH), Val::Px(35.0)),
            position: Rect {
                left: Val::Px((WINDOW_HEIGHT * RESOLUTION - LOADING_BORDER_WIDTH) / 2.0),
                top: Val::Px((WINDOW_HEIGHT - LOADING_BORDER_HEIGHT) / 2.0 - 37.0),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        color: UiColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        let glossary = dictionary.get_glossary();

        parent.spawn_bundle(TextBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },

            text: Text::with_section(
                glossary.loading_scene_text.loading.to_string(),
                TextStyle {
                    font: asset_server.load(dictionary.get_font()),
                    font_size: LOADING_TEXT_FONT_SIZE,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        });
    });
}

fn update_loader(
    mut query: Query<(&mut Loader, &mut Style, &Children)>,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<State<SceneState>>,
) {
    for (mut loader, mut style, children) in query.iter_mut() {
        if loader.current_width < loader.max_width {
            loader.current_width += 3.0;
            style.size.width = Val::Px(loader.current_width);

            let value = (loader.current_width / loader.max_width * 100.0) as usize;
            if value >= 6 {
                let mut text = text_query.get_mut(children[0]).unwrap();
                text.sections[0].value = value.to_string() + "%";
            }
        } else {
            state
                .set(SceneState::MainMenuScene)
                .expect("Couldn't switch state to Main Menu Scene");
        }
    }
}
