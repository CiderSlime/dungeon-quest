use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod cheat;
pub mod feature;
pub mod movement;

pub struct InputHandlePlugin;

impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicMode)
                .with_system(movement::player_movement_handle_system)
                .with_system(feature::pause)
                .with_system(cheat::cheat_move),
        );

        app.add_system_set(
            SystemSet::on_update(SceneState::InGameSurvivalMode)
                .with_system(movement::player_movement_handle_system)
                .with_system(feature::pause),
        );
    }
}
