use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::config::DATA_FILE;
use crate::ingame::resources::base_stats::BaseStats;
use crate::ingame::resources::effect::Effect;
use crate::ingame::resources::skill::Skill;
use crate::ingame::resources::weapon::Weapon;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    elf_base_stats: BaseStats,
    knight_base_stats: BaseStats,
    wizard_base_stats: BaseStats,
    lizard_base_stats: BaseStats,
    weapons: [Weapon; 11],
    effects: [Effect; 11],
    skills: [Skill; 4],
}

impl Data {
    pub fn new() -> Self {
        let data: Data = match File::open(DATA_FILE) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(err) => panic!("Can't find language file: {}", err.to_string()),
        };
        data
    }
}
