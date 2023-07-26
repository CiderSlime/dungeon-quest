use bevy::{prelude::*, window::WindowMode};
use bevy::window::{WindowResolution, WindowResizeConstraints};
use bevy_kira_audio::{AudioPlugin};

use config::*;

mod components;
mod config;
mod materials;
mod plugins;
mod resources;
mod scenes;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_HEIGHT * RESOLUTION, WINDOW_HEIGHT),
                title: TITLE.to_string(),
                position: WindowPosition::At(IVec2::new(MONITOR_WIDTH / 4, MONITOR_HEIGHT / 4)),
                resizable: false,
                resize_constraints: WindowResizeConstraints {
                    min_width: WINDOW_HEIGHT * RESOLUTION,
                    max_width: WINDOW_HEIGHT * RESOLUTION,
                    min_height: WINDOW_HEIGHT,
                    max_height: WINDOW_HEIGHT,
                },
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<resources::setting::Setting>()
        .init_resource::<resources::dictionary::Dictionary>()
        .add_state::<scenes::SceneState>()

        .add_plugin(AudioPlugin)
        .add_startup_system(plugins::music::background_audio_channel_setup)
        .add_system(plugins::music::play_background_music)
        .add_plugin(plugins::camera::CameraPlugin)
        .add_plugin(scenes::loading_scene::LoadingScenePlugin)
        .add_plugin(scenes::main_menu_scene::MainMenuScenePlugin)
        .add_plugin(scenes::highscore_scene::HighscoreScenePlugin)
        .add_plugin(scenes::options_scene::OptionsScenePlugin)
        .add_plugin(scenes::help_scene::HelpScenePlugin)
        .add_plugin(scenes::credits_scene::CreditsScenePlugin)
        .add_plugin(scenes::game_mode_select_scene::GameModeSelectScenePlugin)
        .add_plugin(scenes::hero_select_scene::HeroSelectScenePlugin)
        .add_plugin(scenes::result_scene::ResultScenePlugin)
        .add_plugin(scenes::pause_scene::PauseScenePlugin)
        .add_plugin(scenes::rewards_scene::RewardsScenePlugin)
        .add_plugin(scenes::reward_scene::RewardScenePlugin)
        .add_plugin(plugins::input::InputHandlePlugin)
        .add_plugin(plugins::player::PlayerPlugin)
        .add_plugin(plugins::weapon::WeaponPlugin)
        .add_plugin(plugins::classic_mode::ClassicModePlugin)
        .add_plugin(plugins::classic_mode::ui::ClassicModeUIPlugin)
        .add_plugin(plugins::survival_mode::SurvivalModePlugin)
        .add_plugin(plugins::survival_mode::ui::SurvivalModeUIPlugin)
        .add_plugin(plugins::monster::MonsterPlugin)
        // .add_plugin(plugins::debug::DebugPlugin)ss
        .run();
}
