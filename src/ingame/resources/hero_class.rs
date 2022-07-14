use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HeroClass {
    Elf,
    Knight,
    Wizard,
    Lizard,
}

impl HeroClass {
    pub fn iterator() -> Iter<'static, HeroClass> {
        static HERO_CLASSES: [HeroClass; 4] = [
            HeroClass::Elf,
            HeroClass::Knight,
            HeroClass::Wizard,
            HeroClass::Lizard,
        ];
        HERO_CLASSES.iter()
    }
}
