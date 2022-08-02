use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Inspectable, Copy)]
pub enum EffectType {
    SpeedUp,
    EvasionUp,
    Focus,
    Slow,
    ReduceDamage,
    Disarm,
    Confuse,
    Stun,
}
