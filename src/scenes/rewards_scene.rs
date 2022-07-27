use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::slice::Iter;

use crate::config::*;
use crate::ingame::materials::InGameMaterials;
use crate::ingame::resources::data::Data;
use crate::ingame::resources::dungeon::wave::Wave;
use crate::ingame::resources::hero::hero_class::HeroClass;
use crate::ingame::resources::player::player_effects::PlayerEffects;
use crate::ingame::resources::player::player_skill::PlayerSkill;
use crate::ingame::resources::player::Player;
use crate::ingame::resources::skill::skill_type::SkillType;
use crate::ingame::resources::upgrade::upgrade_controller::UpgradeController;
use crate::ingame::resources::upgrade::upgrade_type::UpgradeType;
use crate::ingame::resources::upgrade::Upgrade;
use crate::ingame::resources::weapon::attack_type::AttackType;
use crate::ingame::resources::weapon::bullet::Bullet;
use crate::ingame::resources::weapon::bullet_controller::BulletController;
use crate::ingame::resources::weapon::weapon_type::WeaponType;
use crate::ingame::weapon::WeaponComponent;
use crate::materials::scenes::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::materials::Materials;
use crate::resources::dictionary::Dictionary;
use crate::scenes::SceneState;

const BOX_TILE_SIZE: f32 = 60.0;
const BOX_WIDTH_TILES: f32 = 4.0;
const BOX_HEIGHT_TILES: f32 = 4.0;

const BOX_ARRAY: [[i8; 4]; 4] = [[0, 1, 1, 2], [3, 4, 4, 5], [3, 4, 4, 5], [6, 7, 7, 8]];

#[derive(Component, Copy, Clone, PartialEq, Eq)]
enum RewardsSceneButton {
    One,
    Two,
    Three,
}

#[derive(Component)]
struct Reward {
    upgrade_type: UpgradeType,
}

impl RewardsSceneButton {
    pub fn iterator() -> Iter<'static, RewardsSceneButton> {
        static BUTTONS: [RewardsSceneButton; 3] = [
            RewardsSceneButton::One,
            RewardsSceneButton::Two,
            RewardsSceneButton::Three,
        ];
        BUTTONS.iter()
    }
}

struct RewardsSceneData {
    user_interface_root: Entity,
}

pub struct RewardsScenePlugin;

impl Plugin for RewardsScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::RewardsScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::RewardsScene).with_system(button_handle_system),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::RewardsScene).with_system(cleanup));
    }
}

fn setup(
    upgrade_controller: Res<UpgradeController>,
    scenes_materials: Res<ScenesMaterials>,
    weapon_query: Query<&WeaponComponent>,
    player_query: Query<&Player>,
    dictionary: Res<Dictionary>,
    materials: Res<Materials>,
    mut commands: Commands,
) {
    let player = player_query.single();
    let weapon_component = weapon_query.single();
    let three_upgrades = upgrade_controller.get_three_upgrades(player, weapon_component.level);

    let user_interface_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            menu_box(parent, &scenes_materials.menu_box_materials);
            buttons(parent, &materials, &dictionary, three_upgrades);
        })
        .insert(Name::new("RewardsUI"))
        .id();

    commands.insert_resource(RewardsSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, rewards_scene_data: Res<RewardsSceneData>) {
    commands
        .entity(rewards_scene_data.user_interface_root)
        .despawn_recursive();
}

fn menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let size: Size<Val> = Size {
        width: Val::Px(BOX_TILE_SIZE),
        height: Val::Px(BOX_TILE_SIZE),
    };

    let start_left = (WINDOW_HEIGHT * RESOLUTION - BOX_TILE_SIZE * BOX_WIDTH_TILES) / 2.0;
    let start_top = (WINDOW_HEIGHT - BOX_TILE_SIZE * BOX_HEIGHT_TILES) / 2.0;

    root.spawn_bundle(NodeBundle {
        ..Default::default()
    })
    .with_children(|parent| {
        for (row_index, row) in BOX_ARRAY.iter().enumerate() {
            for (column_index, value) in row.iter().enumerate() {
                let position: Rect<Val> = Rect {
                    left: Val::Px(start_left + BOX_TILE_SIZE * column_index as f32),
                    top: Val::Px(start_top + BOX_TILE_SIZE * row_index as f32),
                    bottom: Val::Auto,
                    right: Val::Auto,
                };

                let image: Handle<Image> = match value {
                    0 => menu_box_materials.top_right.clone(),
                    1 => menu_box_materials.top_center.clone(),
                    2 => menu_box_materials.top_left.clone(),
                    3 => menu_box_materials.mid_right.clone(),
                    4 => menu_box_materials.mid_center.clone(),
                    5 => menu_box_materials.mid_left.clone(),
                    6 => menu_box_materials.bottom_right.clone(),
                    7 => menu_box_materials.bottom_center.clone(),
                    8 => menu_box_materials.bottom_left.clone(),
                    _ => panic!("Unknown resources"),
                };

                parent.spawn_bundle(NodeBundle {
                    image: UiImage(image),
                    style: Style {
                        position_type: PositionType::Absolute,
                        position,
                        size,
                        ..Default::default()
                    },

                    ..Default::default()
                });
            }
        }
    })
    .insert(Name::new("MenuBox"));
}

fn buttons(
    root: &mut ChildBuilder,
    materials: &Materials,
    dictionary: &Dictionary,
    three_upgrades: Vec<UpgradeType>,
) {
    let font = materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    for (index, button) in RewardsSceneButton::iterator().enumerate() {
        let upgrade_type = three_upgrades[index].clone();

        let value = match upgrade_type {
            UpgradeType::Weapon => glossary.ingame_text.weapon.clone(),
            UpgradeType::Stats => glossary.ingame_text.stats.clone(),
            UpgradeType::Skill => glossary.ingame_text.skill.clone(),
            UpgradeType::Effect => glossary.ingame_text.effect.clone(),
        };

        let top_position = match *button {
            RewardsSceneButton::One => 220.0,
            RewardsSceneButton::Two => 270.0,
            RewardsSceneButton::Three => 320.0,
        };

        root.spawn_bundle(ButtonBundle {
            style: Style {
                position: Rect {
                    left: Val::Px(435.0),
                    top: Val::Px(top_position),
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
                size: Size {
                    width: Val::Px(150.0),
                    height: Val::Px(35.0),
                },
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    value.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 35.0,
                        color: Color::GRAY,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(Reward { upgrade_type })
        .insert(Name::new(value.clone()))
        .insert(button.clone());
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &RewardsSceneButton, &Reward, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut weapon_query: Query<(&mut WeaponComponent, &mut Sprite, &mut Handle<Image>)>,
    mut bullet_controller: ResMut<BulletController>,
    upgrade_controller: Res<UpgradeController>,
    mut player_effects: ResMut<PlayerEffects>,
    ingame_materials: Res<InGameMaterials>,
    mut player_skill: ResMut<PlayerSkill>,
    mut state: ResMut<State<SceneState>>,
    mut player_query: Query<&mut Player>,
    mut text_query: Query<&mut Text>,
    mut wave: ResMut<Wave>,
    data: Res<Data>,
) {
    for (interaction, _button, reward, children) in button_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => text.sections[0].style.color = Color::GRAY,
            Interaction::Hovered => text.sections[0].style.color = Color::BLACK.into(),
            Interaction::Clicked => {
                match reward.upgrade_type {
                    UpgradeType::Weapon => {
                        let hero_class = player_query.single().class.clone();
                        upgrade_weapon(
                            &mut weapon_query,
                            &mut bullet_controller,
                            &ingame_materials,
                            hero_class,
                            &data,
                        );
                    }
                    UpgradeType::Stats => {
                        let upgrade = upgrade_controller.get_stats_upgrade();
                        upgrade_stats(upgrade, &mut player_query);
                    }
                    UpgradeType::Effect => {
                        let upgrade = upgrade_controller.get_effect_upgrade();
                        upgrade_effect(upgrade, &mut player_effects);
                    }
                    UpgradeType::Skill => {
                        let skill_type = player_skill.skill.name.clone();
                        let upgrade = upgrade_controller.get_skill_upgrade(skill_type);
                        upgrade_skill(upgrade, &mut player_skill);
                    }
                }

                wave.next_wave();
                state.pop().unwrap();
            }
        }
    }
}

fn upgrade_stats(upgrade: Upgrade, player_query: &mut Query<&mut Player>) {
    let mut player = player_query.single_mut();

    let stats_upgrade = upgrade.stats_upgrade.unwrap();

    let critical_chance_bonus_upgrade = stats_upgrade.critical_chance_bonus.unwrap_or(0.0);
    let dodge_chance_bonus_upgrade = stats_upgrade.dodge_chance_bonus.unwrap_or(0.0);
    let restore_chance_bonus_upgrade = stats_upgrade.restore_chance_bonus.unwrap_or(0.0);
    let intelligence_bonus_upgrade = stats_upgrade.intelligence_bonus.unwrap_or(0.0);
    let strength_bonus_upgrade = stats_upgrade.strength_bonus.unwrap_or(0.0);
    let max_health_bonus_upgrade = stats_upgrade.max_health_bonus.unwrap_or(0.0);
    let speed_percent_bonus_upgrade = stats_upgrade.speed_percent_bonus.unwrap_or(0.0);
    let speed_bonus_upgrade = speed_percent_bonus_upgrade * player.base_stats.speed;

    player.max_health_points += max_health_bonus_upgrade;
    player.base_stats.critical_chance += critical_chance_bonus_upgrade;
    player.base_stats.dodge_chance += dodge_chance_bonus_upgrade;
    player.base_stats.restore_chance += restore_chance_bonus_upgrade;
    player.intelligence += intelligence_bonus_upgrade;
    player.strength += strength_bonus_upgrade;
    player.base_stats.speed += speed_bonus_upgrade;
}

fn upgrade_effect(upgrade: Upgrade, player_effect: &mut PlayerEffects) {
    let effect_upgrade = upgrade.effect_upgrade.unwrap();

    let mut information = player_effect
        .information
        .iter_mut()
        .find(|effect_information| effect_information.name == effect_upgrade.name)
        .unwrap();

    let duration_bonus = effect_upgrade.duration_bonus.unwrap_or(0);
    let duration_reduce = effect_upgrade.duration_reduce.unwrap_or(0);

    let speed_percent_bonus = effect_upgrade.speed_percent_bonus.unwrap_or(0.0);
    let speed_percent_reduce = effect_upgrade.speed_percent_reduce.unwrap_or(0.0);
    let critical_chance_bonus = effect_upgrade.critical_chance_bonus.unwrap_or(0.0);
    let dodge_chance_bonus = effect_upgrade.dodge_chance_bonus.unwrap_or(0.0);

    let duration = duration_bonus - duration_reduce;
    let bonus =
        speed_percent_bonus - speed_percent_reduce + critical_chance_bonus + dodge_chance_bonus;

    information.duration = if information.duration + duration > 0 {
        information.duration + duration
    } else {
        1
    };

    information.bonus = information.bonus + bonus;
}

fn upgrade_skill(upgrade: Upgrade, player_skill: &mut PlayerSkill) {
    let skill_upgrade = upgrade.skill_upgrade.unwrap();

    let duration_bonus = skill_upgrade.duration_bonus.unwrap_or(0);
    let cooldown_reduce = skill_upgrade.cooldown_reduce.unwrap_or(0);
    let require_monsters_reduce = skill_upgrade.require_monsters_reduce.unwrap_or(0);

    let speed_percent_bonus = skill_upgrade.speed_percent_bonus.unwrap_or(0.0);
    let critical_chance_bonus = skill_upgrade.critical_chance_bonus.unwrap_or(0.0);
    let restore_chance_bonus = skill_upgrade.restore_chance_bonus.unwrap_or(0.0);
    let dodge_chance_bonus = skill_upgrade.dodge_chance_bonus.unwrap_or(0.0);

    let skill_duration = player_skill.skill.duration.unwrap_or(0);
    let skill_cooldown = player_skill.skill.cooldown.unwrap_or(0);
    let speed_percent = player_skill.skill.speed_percent_bonus.unwrap_or(0.0);
    let critical_chance = player_skill.skill.speed_percent_bonus.unwrap_or(0.0);
    let require_monsters = player_skill.skill.require_monsters.unwrap_or(0);
    let restore_chance = player_skill.skill.restore_chance_bonus.unwrap_or(0.0);
    let dodge_chance = player_skill.skill.dodge_chance_bonus.unwrap_or(0.0);

    match player_skill.skill.name {
        SkillType::TimeToHunt => {
            player_skill.skill.duration = Some(skill_duration + duration_bonus);
            player_skill.skill.cooldown = Some(skill_cooldown - cooldown_reduce);
            player_skill.skill.speed_percent_bonus = Some(speed_percent_bonus + speed_percent);
            player_skill.skill.critical_chance_bonus =
                Some(critical_chance + critical_chance_bonus);
        }
        SkillType::Armor => {
            player_skill.skill.require_monsters = Some(require_monsters - require_monsters_reduce);
        }
        SkillType::Thunderstorm => {
            player_skill.skill.cooldown = Some(skill_cooldown - cooldown_reduce);
        }
        SkillType::AnimalInstinct => {
            player_skill.skill.duration = Some(skill_duration + duration_bonus);
            player_skill.skill.cooldown = Some(skill_cooldown - cooldown_reduce);
            player_skill.skill.speed_percent_bonus = Some(speed_percent_bonus + speed_percent);
            player_skill.skill.critical_chance_bonus =
                Some(critical_chance + critical_chance_bonus);
            player_skill.skill.restore_chance_bonus = Some(restore_chance + restore_chance_bonus);
            player_skill.skill.dodge_chance_bonus = Some(dodge_chance + dodge_chance_bonus);
        }
    };
}

fn upgrade_weapon(
    weapon_query: &mut Query<(&mut WeaponComponent, &mut Sprite, &mut Handle<Image>)>,
    mut bullet_controller: &mut BulletController,
    ingame_materials: &InGameMaterials,
    hero_class: HeroClass,
    data: &Data,
) {
    let (mut weapon_component, mut weapon_sprite, mut texture) = weapon_query.single_mut();
    let weapons = data.get_weapons(hero_class.clone());
    let weapon_level = weapon_component.level;
    if weapon_level >= 3 || weapon_level == 2 && hero_class == HeroClass::Elf {
        return;
    } else {
        let weapon_next_level = weapon_level + 1;

        let weapon = weapons
            .iter()
            .find(|weapon| weapon.level == weapon_next_level)
            .expect("Cant' find weapon")
            .clone();

        weapon_component.name = weapon.name.clone();
        weapon_component.level = weapon.level;
        weapon_component.swing_speed = weapon.swing_speed.unwrap_or(0.0);
        weapon_component.cooldown_second = weapon.cooldown.unwrap_or(0);
        weapon_component.attack_type = weapon.attack_type.clone();
        weapon_component.size_width = weapon.width;
        weapon_component.size_height = weapon.height;
        weapon_component.scale = weapon.scale;

        let bullet = weapon.bullet.unwrap_or(Bullet {
            width: 0.0,
            height: 0.0,
            speed: 0.0,
            scale: 0.0,
        });

        bullet_controller.bullet_information = bullet;

        weapon_sprite.custom_size = Some(Vec2::new(
            weapon_component.size_width * weapon.scale,
            weapon_component.size_height * weapon.scale,
        ));

        weapon_sprite.anchor = match weapon.attack_type {
            AttackType::Swing => Anchor::BottomCenter,
            AttackType::Throw => Anchor::BottomCenter,
            AttackType::Shoot => Anchor::Center,
        };

        *texture = match weapon.name {
            WeaponType::Spear => ingame_materials.weapons_materials.spear.clone(),
            WeaponType::Sword => ingame_materials.weapons_materials.small_wand.clone(),
            WeaponType::BigMachete => ingame_materials.weapons_materials.machete.clone(),
            WeaponType::MagicWand => ingame_materials.weapons_materials.magic_wand.clone(),
            WeaponType::MagicSword => ingame_materials.weapons_materials.magic_sword.clone(),
            WeaponType::Mace => ingame_materials.weapons_materials.mace.clone(),
            WeaponType::BigHammer => ingame_materials.weapons_materials.big_hammer.clone(),
            WeaponType::Bow => ingame_materials.weapons_materials.bow.clone(),
            WeaponType::ShortSword => ingame_materials.weapons_materials.short_sword.clone(),
            WeaponType::SmallWand => ingame_materials.weapons_materials.small_wand.clone(),
            WeaponType::SmallHammer => ingame_materials.weapons_materials.small_hammer.clone(),
        };
    }
}
