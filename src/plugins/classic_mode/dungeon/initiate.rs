use bevy::prelude::*;

use crate::plugins::classic_mode::ClassicModeData;
use crate::resources::dungeon::Dungeon;
use crate::resources::game_mode::GameMode;
use crate::resources::monster::monster_spawn_controller::MonsterSpawnController;
use crate::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::scenes::SceneState;

pub fn initiate_classic_mode(mut commands: Commands, mut state: ResMut<State<SceneState>>) {
    let dungeon = Dungeon::new();

    let player_dungeon_stats = PlayerDungeonStats {
        current_room_position: dungeon.current_floor.current_position,
        is_room_cleared: true,
        current_floor_index: 0,
    };

    let classic_mode_data = ClassicModeData {
        doors: None,
        walls: None,
        ground: None,
        end_point: None,
    };

    let monster_spawn_controller = MonsterSpawnController {
        game_mode: GameMode::ClassicMode,
        max_avalible_monsters: 6,
        require_monster: 0,
        alive_monsters: 0,
    };

    commands.insert_resource(dungeon);
    commands.insert_resource(player_dungeon_stats);
    commands.insert_resource(classic_mode_data);
    commands.insert_resource(monster_spawn_controller);

    state
        .set(SceneState::InGameClassicMode)
        .expect("Can't change to InGame Classic Mode Scene");
}
