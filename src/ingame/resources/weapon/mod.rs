use serde::{Deserialize, Serialize};

use crate::ingame::resources::effect::effect_type::EffectType;

pub mod weapon_type;

use weapon_type::WeaponType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weapon {
    pub name: WeaponType,
    pub strength: f32,
    pub intelligence: f32,
    pub effect: Option<EffectType>,
}
