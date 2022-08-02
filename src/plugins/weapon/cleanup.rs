use bevy::prelude::*;

use crate::plugins::weapon::WeaponEntity;

pub fn cleanup_weapon(mut commands: Commands, weapon_entity: Res<WeaponEntity>) {
    commands.entity(weapon_entity.entity).despawn_recursive();
}
