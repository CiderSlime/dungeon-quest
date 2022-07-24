use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::ingame::resources::fixed::gender::Gender;
use crate::ingame::resources::fixed::hero_class::HeroClass;
use crate::ingame::resources::game_mode::GameMode;
use crate::ingame::resources::stored_profile::StoredProfile;
use crate::scenes::hero_select_scene::HeroSelectSceneButton;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub game_mode: GameMode,
    pub hero_class: HeroClass,
    pub gender: Gender,
    pub total_killed_monsters: usize,
    pub total_cleared_rooms: usize,
    pub total_cleared_waves: usize,
    pub start_time: String,
    pub end_time: String,
    pub is_run_completed: bool,
    pub is_run_finished: bool,
}

impl Profile {
    pub fn new() -> Self {
        let start_time: DateTime<Local> = Local::now();

        Profile {
            name: String::new(),
            game_mode: GameMode::ClassicMode,
            hero_class: HeroClass::Elf,
            gender: Gender::Male,
            total_cleared_rooms: 0,
            total_killed_monsters: 0,
            total_cleared_waves: 0,
            end_time: start_time.to_rfc3339(),
            start_time: start_time.to_rfc3339(),
            is_run_completed: false,
            is_run_finished: false,
        }
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
    }

    pub fn set_hero(&mut self, button: HeroSelectSceneButton) {
        match button {
            HeroSelectSceneButton::MaleElf => {
                self.hero_class = HeroClass::Elf;
                self.gender = Gender::Male;
            }
            HeroSelectSceneButton::FemaleElf => {
                self.hero_class = HeroClass::Elf;
                self.gender = Gender::Female;
            }
            HeroSelectSceneButton::MaleKnight => {
                self.hero_class = HeroClass::Knight;
                self.gender = Gender::Male;
            }
            HeroSelectSceneButton::FemaleKnight => {
                self.hero_class = HeroClass::Knight;
                self.gender = Gender::Female;
            }
            HeroSelectSceneButton::MaleLizard => {
                self.hero_class = HeroClass::Lizard;
                self.gender = Gender::Male;
            }
            HeroSelectSceneButton::FemaleLizard => {
                self.hero_class = HeroClass::Lizard;
                self.gender = Gender::Female;
            }
            HeroSelectSceneButton::MaleWizard => {
                self.hero_class = HeroClass::Wizard;
                self.gender = Gender::Male;
            }
            HeroSelectSceneButton::FemaleWizard => {
                self.hero_class = HeroClass::Wizard;
                self.gender = Gender::Female;
            }
        }
    }

    pub fn set_name(&mut self, user_name: String) {
        self.name = user_name;
    }

    pub fn convert_to_stored_profile(&self) -> StoredProfile {
        let start_time = DateTime::parse_from_rfc3339(self.start_time.clone().as_str())
            .expect("Error convert time");

        let end_time = DateTime::parse_from_rfc3339(self.end_time.clone().as_str())
            .expect("Error convert time");

        let diff_time = end_time - start_time;
        let diff_time_second = diff_time.num_seconds();

        StoredProfile {
            name: self.name.clone(),
            game_mode: self.game_mode.clone(),
            hero_class: self.hero_class.clone(),
            gender: self.gender.clone(),
            total_cleared_rooms: self.total_cleared_rooms,
            total_killed_monsters: self.total_killed_monsters,
            total_cleared_waves: self.total_cleared_waves,
            date: self.start_time.clone(),
            playtime: diff_time_second,
        }
    }
}
