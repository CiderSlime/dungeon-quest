use bevy::prelude::*;

use crate::resources::weapon::attack_type::AttackType;
use crate::resources::weapon::weapon_type::WeaponType;
use crate::resources::weapon::Weapon;

#[derive(Component, Copy, Clone)]
pub struct WeaponComponent {
    pub attack_type: AttackType,
    pub name: WeaponType,
    pub level: u8,
    pub size_height: f32,
    pub size_width: f32,
    pub scale: f32,
    pub strength: f32,
    pub intelligence: f32,
}

impl WeaponComponent {
    pub fn upgrade_weapon(&mut self, weapon: &Weapon) {
        self.strength = weapon.strength;
        self.intelligence = weapon.intelligence;
        self.attack_type = weapon.attack_type.clone();
        self.name = weapon.name.clone();
        self.level = weapon.level;
        self.size_width = weapon.width;
        self.size_height = weapon.height;
        self.scale = weapon.scale;
    }
}
